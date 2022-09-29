#![feature(generic_associated_types)]

//mod common;

//use debugger::CliState;

use clap_num::maybe_hex;
use probe_rs::flashing::{erase_all, BinOptions};
use rand::RngCore;

use anyhow::Result;
use btmesh_common::{
    address::{LabelUuid, UnicastAddress},
    crypto::{application::ApplicationKey, device::DeviceKey, network::NetworkKey},
    location::{FRONT, LEFT, RIGHT},
    CompanyIdentifier, Composition, ElementDescriptor, IvIndex, IvUpdateFlag, ModelIdentifier,
    ProductIdentifier, VersionIdentifier,
};
use btmesh_driver::{
    stack::provisioned::{
        network::DeviceInfo,
        secrets::{application::ApplicationKeys, network::NetworkKeys, Secrets},
        NetworkState,
    },
    storage::ProvisionedConfiguration,
};
use btmesh_models::foundation::configuration::{
    model_publication::{
        PublicationDetails, PublishAddress, PublishPeriod, PublishRetransmit, Resolution,
    },
    AppKeyIndex, NetKeyIndex,
};
use probe_rs_cli_util::{
    clap,
    clap::Parser,
    common_options::{CargoOptions, FlashOptions, ProbeOptions},
    flash::run_flash_download,
};
use rand::rngs::OsRng;
use std::path::Path;

#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
enum Cli {
    Provision {
        #[structopt(flatten)]
        common: ProbeOptions,

        /// The address given to the node
        #[clap(long, parse(try_from_str=maybe_hex))]
        flash_address: u64,

        /// The address given to the node
        #[clap(long, parse(try_from_str=maybe_hex))]
        node_address: u16,

        /// Network key (16 byte hex)
        #[clap(long)]
        network_key: String,

        /// Device key (16 byte hex)
        #[clap(long)]
        device_key: Option<String>,

        /// Application key (16 byte hex)
        #[clap(long)]
        application_key: String,

        /// Whether to erase the entire chip before downloading
        #[structopt(long)]
        chip_erase: bool,
    },
    /// Erase the provisioning info of the device
    Erase {
        #[structopt(flatten)]
        common: ProbeOptions,
    },
}

fn main() -> Result<()> {
    // Initialize the logging backend.
    pretty_env_logger::init();

    let matches = Cli::parse();

    match matches {
        Cli::Provision {
            common,
            flash_address,
            node_address,
            network_key,
            device_key,
            application_key,
            chip_erase,
        } => {
            let left_address = (node_address + 1).to_be_bytes();
            let right_address = (node_address + 1).to_be_bytes();
            let base_address = node_address.to_be_bytes();
            let base_address = UnicastAddress::parse(base_address).unwrap();
            let left_address = UnicastAddress::parse(left_address).unwrap();
            let right_address = UnicastAddress::parse(right_address).unwrap();
            let device_info = DeviceInfo::new(base_address, 3);

            let device_key = device_key
                .map(|k| decode_key(&k).unwrap())
                .unwrap_or_else(|| {
                    let mut key = [0u8; 16];
                    OsRng.fill_bytes(&mut key);
                    key
                });
            let device_key = DeviceKey::new(device_key);

            let network_key = NetworkKey::new(decode_key(&network_key).unwrap()).unwrap();
            let mut network_keys = NetworkKeys::default();
            network_keys.set(0, network_key).unwrap();

            let app_key = ApplicationKey::new(decode_key(&application_key).unwrap()).unwrap();
            let app_key_idx = AppKeyIndex::new(0);
            let mut app_keys = ApplicationKeys::default();
            app_keys
                .add(app_key_idx, NetKeyIndex::new(0), app_key)
                .unwrap();
            let secrets = Secrets::new(device_key, network_keys, app_keys);

            let network_state = NetworkState::new(IvIndex::new(0), IvUpdateFlag::parse(0));

            let mut front = ElementDescriptor::new(FRONT);
            front.add_model(ModelIdentifier::SIG(0x1000));
            front.add_model(ModelIdentifier::SIG(0x1002));
            front.add_model(ModelIdentifier::SIG(0x100C));
            front.add_model(ModelIdentifier::SIG(0x1101));

            let mut left = ElementDescriptor::new(LEFT);
            left.add_model(ModelIdentifier::SIG(0x1001));
            let mut right = ElementDescriptor::new(RIGHT);
            right.add_model(ModelIdentifier::SIG(0x1001));
            let mut composition: Composition<()> = Composition::new(
                CompanyIdentifier(0x0003),
                ProductIdentifier(0x0001),
                VersionIdentifier(0x0001),
            );
            composition.add_element(front).unwrap();
            composition.add_element(left).unwrap();
            composition.add_element(right).unwrap();

            let mut config: ProvisionedConfiguration = (device_info, secrets, network_state).into();

            // Bind all models in front element
            for i in 0..4 {
                config
                    .bindings_mut()
                    .bind(
                        &composition,
                        0,
                        composition[0][i].model_identifier,
                        app_key_idx,
                    )
                    .unwrap();
            }

            // Bind first element of left and right elements
            for i in 1..3 {
                config
                    .bindings_mut()
                    .bind(
                        &composition,
                        i,
                        composition[i][0].model_identifier,
                        app_key_idx,
                    )
                    .unwrap();
            }

            // Battery service publication
            config
                .publications_mut()
                .set(
                    &composition,
                    0,
                    pub_set(
                        base_address,
                        app_key_idx,
                        composition[0][2].model_identifier,
                        Some(60),
                    ),
                )
                .unwrap();

            // Sensor publication
            config
                .publications_mut()
                .set(
                    &composition,
                    0,
                    pub_set(
                        base_address,
                        app_key_idx,
                        composition[0][3].model_identifier,
                        Some(1),
                    ),
                )
                .unwrap();

            // Button publications
            config
                .publications_mut()
                .set(
                    &composition,
                    1,
                    pub_set(
                        left_address,
                        app_key_idx,
                        composition[1][0].model_identifier,
                        None,
                    ),
                )
                .unwrap();

            config
                .publications_mut()
                .set(
                    &composition,
                    2,
                    pub_set(
                        right_address,
                        app_key_idx,
                        composition[2][0].model_identifier,
                        None,
                    ),
                )
                .unwrap();

            provision(common, flash_address, chip_erase, config)?;
            Ok(())
        }
        Cli::Erase { common } => erase(&common),
    }
}

fn pub_set(
    address: UnicastAddress,
    app_key_index: AppKeyIndex,
    model: ModelIdentifier,
    period_secs: Option<u8>,
) -> PublicationDetails {
    let label = LabelUuid::new(decode_key("f0bfd803cde184133096f003ea4a3dc2").unwrap()).unwrap();
    let pub_address = PublishAddress::Label(label);
    let publish_period = period_secs
        .map(|period_secs| PublishPeriod::new(period_secs, Resolution::Seconds1))
        .unwrap_or(0.into());
    let rxt = PublishRetransmit::from(0);
    PublicationDetails {
        element_address: address,
        publish_address: pub_address,
        app_key_index,
        credential_flag: false,
        publish_ttl: None,
        publish_period: publish_period,
        publish_retransmit: rxt,
        model_identifier: model,
    }
}

fn provision(
    common: ProbeOptions,
    base_address: u64,
    do_chip_erase: bool,
    config: ProvisionedConfiguration,
) -> Result<()> {
    let mut session = common.simple_attach()?;

    let file: Vec<u8> = postcard::to_allocvec(&config)?;
    let mut loader = session.target().flash_loader();

    let options = BinOptions {
        base_address: Some(base_address),
        skip: 0,
    };

    let mut file = std::io::Cursor::new(file);
    loader.load_bin_data(&mut file, options)?;

    run_flash_download(
        &mut session,
        Path::new("inmemory"),
        &FlashOptions {
            version: false,
            list_chips: false,
            list_probes: false,
            disable_progressbars: false,
            disable_double_buffering: false,
            reset_halt: false,
            log: None,
            restore_unwritten: false,
            flash_layout_output_path: None,
            elf: None,
            work_dir: None,
            cargo_options: CargoOptions::default(),
            probe_options: common,
        },
        loader,
        do_chip_erase,
    )?;

    Ok(())
}

fn erase(common: &ProbeOptions) -> Result<()> {
    let mut session = common.simple_attach()?;

    erase_all(&mut session)?;

    Ok(())
}

pub fn decode_key(s: &str) -> Result<[u8; 16], anyhow::Error> {
    let s = s.trim();
    let v: Vec<u8> = hex::decode(s)?;
    let r: [u8; 16] = v.try_into().unwrap();
    Ok(r)
}

/*
enum DownloadFileType {
    Elf,
    Hex,
    Bin,
}

impl DownloadFileType {
    fn into(self, base_address: Option<u64>, skip: Option<u32>) -> Format {
        match self {
            DownloadFileType::Elf => Format::Elf,
            DownloadFileType::Hex => Format::Hex,

        }
    }
}
*/
