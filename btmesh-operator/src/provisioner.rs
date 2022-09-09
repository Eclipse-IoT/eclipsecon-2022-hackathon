use cloudevents::{event::AttributeValue, Data, Event};
use drogue_client::{
    core::v1::{ConditionStatus, Conditions},
    dialect,
    meta::v1::CommonMetadataMut,
    registry::v1::Device,
    Section, Translator,
};
use futures::stream::StreamExt;
use paho_mqtt as mqtt;
use serde::{Deserialize, Serialize};
use tokio::{join, time::Duration};

pub type DrogueClient = drogue_client::registry::v1::Client;

pub struct Operator {
    client: mqtt::AsyncClient,
    group_id: Option<String>,
    application: String,
    registry: DrogueClient,
    interval: Duration,
}

impl Operator {
    pub fn new(
        client: mqtt::AsyncClient,
        group_id: Option<String>,
        application: String,
        registry: DrogueClient,
        interval: Duration,
    ) -> Self {
        Self {
            client,
            group_id,
            application,
            registry,
            interval,
        }
    }

    pub async fn provision_devices(&self, mut devices: Vec<Device>) {
        for device in devices.iter_mut() {
            if let Some(Ok(spec)) = device.section::<BtMeshSpec>() {
                let status: BtMeshStatus =
                    if let Some(Ok(status)) = device.section::<BtMeshStatus>() {
                        status
                    } else {
                        BtMeshStatus {
                            address: None,
                            conditions: Default::default(),
                            state: BtMeshDeviceState::Provisioning { error: None },
                        }
                    };

                let topic = format!(
                    "command/{}/{}/btmesh",
                    self.application, device.metadata.name,
                );
                log::info!("Using topic {} for commands", topic);
                let uuid = spec.device.to_ascii_lowercase();
                Self::ensure_alias(device, &uuid);
                self.update_device(device, status.clone()).await;

                if device.metadata.deletion_timestamp.is_none() {
                    log::info!(
                        "Device {} is active, setting finalizer and sending provisioning message",
                        device.metadata.name
                    );
                    device.metadata.ensure_finalizer("btmesh-operator");

                    // Send provisioning command for this device
                    if let BtMeshDeviceState::Provisioning { error: _ } = status.state {
                        if let Ok(command) = serde_json::to_vec(&BtMeshCommand {
                            command: BtMeshOperation::Provision {
                                device: uuid.clone(),
                            },
                        }) {
                            let message = mqtt::Message::new(topic, &command[..], 1);
                            if let Err(e) = self.client.publish(message).await {
                                log::warn!("Error publishing command back to device: {:?}", e);
                            }
                        }
                    }
                } else {
                    log::info!(
                        "Device {} is being deleted, sending reset command",
                        device.metadata.name
                    );
                    if let Some(address) = &status.address {
                        if let Ok(command) = serde_json::to_vec(&BtMeshCommand {
                            command: BtMeshOperation::Reset { address: *address },
                        }) {
                            let message = mqtt::Message::new(topic, &command[..], 1);
                            if let Err(e) = self.client.publish(message).await {
                                log::warn!("Error publishing command back to device: {:?}", e);
                            }
                        }
                    }
                }
            }
        }
    }

    pub async fn update_device(&self, device: &mut Device, status: BtMeshStatus) {
        if let Ok(_) = device.set_section::<BtMeshStatus>(status) {
            match self.registry.update_device(&device).await {
                Ok(_) => log::debug!("Device {} status updated", device.metadata.name),
                Err(e) => {
                    log::warn!(
                        "Device {} status update error: {:?}",
                        device.metadata.name,
                        e
                    );
                }
            }
        }
    }

    pub async fn reconcile_devices(&self) {
        log::info!("Reconciling devices with interval {:?}", self.interval);
        loop {
            let devices = self
                .registry
                .list_devices(&self.application, None)
                .await
                .unwrap_or(None)
                .unwrap_or(Vec::new());

            self.provision_devices(devices).await;
            tokio::time::sleep(self.interval).await;
        }
    }

    pub async fn run(&mut self) -> Result<(), anyhow::Error> {
        if let Some(group_id) = &self.group_id {
            self.client.subscribe(
                format!("$shared/{}/app/{}", &group_id, &self.application),
                1,
            );
        } else {
            self.client
                .subscribe(format!("app/{}", &self.application), 1);
        }

        let stream = self.client.get_stream(100);
        join!(self.reconcile_devices(), self.process_events(stream));
        Ok(())
    }

    fn ensure_alias(device: &mut Device, alias: &str) {
        let mut aliases: Vec<String> = device
            .spec
            .get("alias")
            .map(|s| {
                if let Some(v) = s.as_array() {
                    v.iter()
                        .map(|e| e.as_str().map(|s| s.to_string()))
                        .flatten()
                        .collect()
                } else {
                    Vec::new()
                }
            })
            .unwrap_or(Vec::new());

        let alias = alias.to_string();
        if !aliases.contains(&alias) {
            aliases.push(alias);
        }

        device
            .spec
            .insert("alias".to_string(), serde_json::json!(aliases));
    }

    pub async fn process_events(
        &self,
        mut stream: paho_mqtt::AsyncReceiver<Option<mqtt::Message>>,
    ) {
        log::info!("Processing events events");
        loop {
            if let Some(m) = stream.next().await {
                if let Some(m) = m {
                    match serde_json::from_slice::<Event>(m.payload()) {
                        Ok(e) => {
                            let mut device = String::new();
                            let mut subject = String::new();
                            for a in e.iter() {
                                log::trace!("Attribute {:?}", a);
                                if a.0 == "subject" {
                                    if let AttributeValue::String(s) = a.1 {
                                        subject = s.to_string();
                                    }
                                } else if a.0 == "device" {
                                    if let AttributeValue::String(d) = a.1 {
                                        device = d.to_string();
                                    }
                                }
                            }

                            if subject == "devices" {
                                log::debug!("Got event on devices channel: {:?}", e);
                                let devices = self
                                    .registry
                                    .list_devices(&self.application, None)
                                    .await
                                    .unwrap_or(None)
                                    .unwrap_or(Vec::new());

                                self.provision_devices(devices).await;
                            } else if subject == "btmesh" {
                                log::debug!("Got event on btmesh channel: {:?}", e);
                                let device =
                                    self.registry.get_device(&self.application, device).await;
                                let event: Option<BtMeshEvent> = match e.data() {
                                    Some(Data::Json(v)) => serde_json::from_value(v.clone())
                                        .map(|e| Some(e))
                                        .unwrap_or(None),
                                    _ => None,
                                };

                                if let (Some(event), Ok(Some(mut device))) = (event, device) {
                                    device.metadata.ensure_finalizer("btmesh-operator");
                                    let mut status: BtMeshStatus = if let Some(Ok(status)) =
                                        device.section::<BtMeshStatus>()
                                    {
                                        status
                                    } else {
                                        BtMeshStatus {
                                            address: None,
                                            conditions: Default::default(),
                                            state: event.status.clone(),
                                        }
                                    };

                                    match &event.status {
                                        BtMeshDeviceState::Reset { error } => {
                                            if let Some(error) = error {
                                                let mut condition = ConditionStatus::default();
                                                condition.status = Some(true);
                                                condition.reason =
                                                    Some("Error resetting device".to_string());
                                                condition.message = Some(error.clone());
                                                status.conditions.update("Provisioned", condition);
                                                status.conditions.update("Provisioning", false);
                                            } else {
                                                status.conditions.update("Provisioned", false);
                                                status.conditions.update("Provisioning", false);
                                                device.metadata.remove_finalizer("btmesh-operator");
                                            }
                                        }
                                        // If we're provisioned, update the status and insert alias in spec if its not already there
                                        BtMeshDeviceState::Provisioned { address } => {
                                            status.conditions.update("Provisioned", true);
                                            status.conditions.update("Provisioning", false);
                                            let a = address.to_le_bytes();
                                            let alias = format!("{:02x}{:02x}", a[0], a[1]);
                                            Self::ensure_alias(&mut device, &alias);
                                        }
                                        BtMeshDeviceState::Provisioning { error } => {
                                            status.conditions.update("Provisioning", true);

                                            let mut condition = ConditionStatus::default();
                                            if let Some(error) = error {
                                                condition.status = Some(false);
                                                condition.reason =
                                                    Some("Error provisioning device".to_string());
                                                condition.message = Some(error.clone());
                                            }

                                            status.conditions.update("Provisioned", condition);
                                        }
                                    }
                                    self.update_device(&mut device, status).await;
                                }
                            }
                        }
                        Err(e) => {
                            log::warn!("Error parsing event: {:?}", e);
                            break;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BtMeshEvent {
    pub status: BtMeshDeviceState,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BtMeshCommand {
    pub command: BtMeshOperation,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BtMeshOperation {
    #[serde(rename = "provision")]
    Provision { device: String },
    #[serde(rename = "reset")]
    Reset { address: u16 },
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum BtMeshDeviceState {
    #[serde(rename = "provisioning")]
    Provisioning { error: Option<String> },

    #[serde(rename = "provisioned")]
    Provisioned { address: u16 },

    #[serde(rename = "reset")]
    Reset { error: Option<String> },
}

dialect!(BtMeshSpec [Section::Spec => "btmesh"]);

#[derive(Serialize, Deserialize, Debug)]
pub struct BtMeshSpec {
    pub device: String,
}

dialect!(BtMeshStatus [Section::Status => "btmesh"]);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BtMeshStatus {
    pub conditions: Conditions,
    pub state: BtMeshDeviceState,
    pub address: Option<u16>,
}
