use btmesh_common::address::UnicastAddress;
use btmesh_common::{InsufficientBuffer, IvUpdateFlag, KeyRefreshFlag, ParseError};
use core::convert::TryInto;
use heapless::Vec;
use p256::elliptic_curve::sec1::{FromEncodedPoint, ToEncodedPoint};

use crate::PDU;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod advertising;
pub mod generic;

#[derive(Clone, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProvisioningPDU {
    Invite(Invite),
    Capabilities(Capabilities),
    Start(Start),
    PublicKey(PublicKey),
    InputComplete,
    Confirmation(Confirmation),
    Random(Random),
    Data(Data),
    Complete,
    Failed(Failed),
}

impl From<ProvisioningPDU> for PDU {
    fn from(inner: ProvisioningPDU) -> Self {
        PDU::Provisioning(inner)
    }
}

#[derive(Clone, Default, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Invite {
    pub attention_duration: u8,
}

impl Invite {
    pub fn parse(data: &[u8]) -> Result<Self, ParseError> {
        if data.len() == 2 && data[0] == ProvisioningPDU::INVITE {
            Ok(Self {
                attention_duration: data[1],
            })
        } else {
            Err(ParseError::InvalidPDUFormat)
        }
    }

    pub fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        xmit.push(ProvisioningPDU::INVITE)?;
        xmit.push(self.attention_duration)?;
        Ok(())
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Default, Hash, Debug)]
pub struct Capabilities {
    pub number_of_elements: u8,
    pub algorithms: Algorithms,
    pub public_key_type: PublicKeyType,
    pub static_oob_type: StaticOOBType,
    pub output_oob_size: OOBSize,
    pub output_oob_action: OutputOOBActions,
    pub input_oob_size: OOBSize,
    pub input_oob_action: InputOOBActions,
}

impl Capabilities {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        if data.len() == 12 && data[0] == ProvisioningPDU::CAPABILITIES {
            let number_of_elements = data[1];
            let algorithms = Algorithms::parse(u16::from_be_bytes([data[2], data[3]]))?;
            let public_key_type = PublicKeyType::parse(data[4])?;
            let static_oob_type = StaticOOBType::parse(data[5])?;
            let output_oob_size = OOBSize::parse(data[6])?;
            let output_oob_action =
                OutputOOBActions::parse(u16::from_be_bytes([data[7], data[8]]))?;
            let input_oob_size = OOBSize::parse(data[9])?;
            let input_oob_action =
                InputOOBActions::parse(u16::from_be_bytes([data[10], data[11]]))?;

            Ok(Self {
                number_of_elements,
                algorithms,
                public_key_type,
                static_oob_type,
                output_oob_size,
                output_oob_action,
                input_oob_size,
                input_oob_action,
            })
        } else {
            Err(ParseError::InvalidPDUFormat)
        }
    }

    pub fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        xmit.push(ProvisioningPDU::CAPABILITIES)?;
        xmit.push(self.number_of_elements)?;
        self.algorithms.emit(xmit)?;
        self.public_key_type.emit(xmit)?;
        self.static_oob_type.emit(xmit)?;
        self.output_oob_size.emit(xmit)?;
        self.output_oob_action.emit(xmit)?;
        self.input_oob_size.emit(xmit)?;
        self.input_oob_action.emit(xmit)?;
        Ok(())
    }
}

#[derive(Clone, Default, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Start {
    pub algorithm: Algorithm,
    pub public_key: PublicKeySelected,
    pub authentication_method: AuthenticationMethod,
    pub authentication_action: OOBAction,
    pub authentication_size: OOBSize,
}

impl Start {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        if data.len() == 6 && data[0] == ProvisioningPDU::START {
            let algorithm = Algorithm::parse(data[1])?;
            let public_key = PublicKeySelected::parse(data[2])?;
            let authentication_method = AuthenticationMethod::parse(data[3])?;
            let authentication_action = OOBAction::parse(&authentication_method, data[4])?;
            let authentication_size =
                Self::parse_authentication_size(&authentication_method, data[5])?;
            Ok(Self {
                algorithm,
                public_key,
                authentication_method,
                authentication_action,
                authentication_size,
            })
        } else {
            Err(ParseError::InvalidPDUFormat)
        }
    }

    fn parse_authentication_size(
        method: &AuthenticationMethod,
        octet: u8,
    ) -> Result<OOBSize, ParseError> {
        match method {
            AuthenticationMethod::No | AuthenticationMethod::Static => {
                if octet != 0 {
                    Err(ParseError::InvalidValue)
                } else {
                    Ok(OOBSize::NotSupported)
                }
            }
            AuthenticationMethod::Output | AuthenticationMethod::Input => {
                if octet == 0 {
                    Err(ParseError::InvalidPDUFormat)
                } else {
                    OOBSize::parse(octet)
                }
            }
        }
    }

    pub fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        xmit.push(ProvisioningPDU::START)?;
        self.algorithm.emit(xmit)?;
        self.public_key.emit(xmit)?;
        self.authentication_method.emit(xmit)?;
        self.authentication_action.emit(xmit)?;
        self.authentication_size.emit(xmit)?;
        Ok(())
    }
}

#[derive(Copy, Clone, Hash, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PublicKey {
    pub x: [u8; 32],
    pub y: [u8; 32],
}

impl PublicKey {
    pub fn parse(data: &[u8]) -> Result<Self, ParseError> {
        if data.len() != 65 && data[0] != ProvisioningPDU::PUBLIC_KEY {
            Err(ParseError::InvalidPDUFormat)
        } else {
            let x = data[1..33].try_into()?;
            let y = data[33..65].try_into()?;
            Ok(PublicKey { x, y })
        }
    }

    pub fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        xmit.push(ProvisioningPDU::PUBLIC_KEY)?;
        xmit.extend_from_slice(&self.x)?;
        xmit.extend_from_slice(&self.y)?;
        Ok(())
    }
}

impl TryFrom<p256::PublicKey> for PublicKey {
    type Error = ParseError;
    fn try_from(pk: p256::PublicKey) -> Result<PublicKey, Self::Error> {
        let xy = pk.to_encoded_point(false);
        let x = xy.x().unwrap();
        let y = xy.y().unwrap();
        Ok(PublicKey {
            x: <[u8; 32]>::try_from(x.as_slice())?,
            y: <[u8; 32]>::try_from(y.as_slice())?,
        })
    }
}

impl TryFrom<&PublicKey> for p256::PublicKey {
    type Error = ParseError;
    fn try_from(pk: &PublicKey) -> Result<p256::PublicKey, Self::Error> {
        let result = p256::PublicKey::from_encoded_point(
            &p256::EncodedPoint::from_affine_coordinates(&pk.x.into(), &pk.y.into(), false),
        );
        if result.is_some().into() {
            Ok(result.unwrap())
        } else {
            Err(ParseError::InvalidValue)
        }
    }
}

#[derive(Clone, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Confirmation {
    pub confirmation: [u8; 16],
}

impl Confirmation {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        if data.len() != 17 && data[0] != ProvisioningPDU::CONFIRMATION {
            Err(ParseError::InvalidPDUFormat)
        } else {
            Ok(Self {
                confirmation: data[1..]
                    .try_into()
                    .map_err(|_| ParseError::InvalidLength)?,
            })
        }
    }

    fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        xmit.push(ProvisioningPDU::CONFIRMATION)?;
        xmit.extend_from_slice(&self.confirmation)?;
        Ok(())
    }
}

#[derive(Clone, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Random {
    pub random: [u8; 16],
}

impl Random {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        if data.len() != 17 && data[0] != ProvisioningPDU::RANDOM {
            Err(ParseError::InvalidPDUFormat)
        } else {
            Ok(Self {
                random: data[1..]
                    .try_into()
                    .map_err(|_| ParseError::InvalidLength)?,
            })
        }
    }

    fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        xmit.push(ProvisioningPDU::RANDOM)?;
        xmit.extend_from_slice(&self.random)?;
        Ok(())
    }
}

#[derive(Clone, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Data {
    pub encrypted: [u8; 25],
    pub mic: [u8; 8],
}

impl Data {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        if data.len() != 34 && data[0] != ProvisioningPDU::DATA {
            Err(ParseError::InvalidPDUFormat)
        } else {
            Ok(Self {
                encrypted: data[1..26]
                    .try_into()
                    .map_err(|_| ParseError::InvalidLength)?,
                mic: data[26..34]
                    .try_into()
                    .map_err(|_| ParseError::InvalidLength)?,
            })
        }
    }
    fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        xmit.push(ProvisioningPDU::DATA)?;
        xmit.extend_from_slice(&self.encrypted)?;
        xmit.extend_from_slice(&self.mic)?;
        Ok(())
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Default, Debug)]
/// The decrypted provisioning data wrapped in `Data` above.
pub struct ProvisioningData {
    pub network_key: [u8; 16],
    // TODO: pub key_index: NetKeyIndex,
    pub key_refresh_flag: KeyRefreshFlag,
    pub iv_update_flag: IvUpdateFlag,
    pub iv_index: u32,
    pub unicast_address: UnicastAddress,
}

impl ProvisioningData {
    pub fn parse(data: &[u8]) -> Result<Self, ParseError> {
        if data.len() < 25 {
            Err(ParseError::InvalidLength)
        } else {
            let network_key = &data[0..16];
            // TODO: let key_index = NetKeyIndex::new(u16::from_be_bytes([data[16], data[17]]));
            let flags = data[18];
            let iv_index = u32::from_be_bytes([data[19], data[20], data[21], data[22]]);
            let unicast_address = UnicastAddress::parse([data[23], data[24]])?;

            Ok(Self {
                network_key: network_key
                    .try_into()
                    .map_err(|_| ParseError::InvalidLength)?,
                // TODO: key_index,
                key_refresh_flag: KeyRefreshFlag::parse(flags & 0b00000001),
                iv_update_flag: IvUpdateFlag::parse(flags & 0b00000010),
                iv_index,
                unicast_address,
            })
        }
    }
    pub fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        xmit.extend_from_slice(&self.network_key)?;
        xmit.extend_from_slice(&[0; 2])?; // TODO: key_index
        let mut flags = 0;
        self.key_refresh_flag.emit(&mut flags);
        self.iv_update_flag.emit(&mut flags);
        xmit.push(flags)?;
        xmit.extend_from_slice(&self.iv_index.to_be_bytes())?;
        xmit.extend_from_slice(&self.unicast_address.as_bytes())?;
        Ok(())
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for ProvisioningData {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(
            fmt,
            "ProvisioningData( network_key={:x}, flags={}:{}, iv_index={}, unicast_address={:x}",
            self.network_key,
            //self.key_index,
            self.key_refresh_flag,
            self.iv_update_flag,
            self.iv_index,
            self.unicast_address,
        )
    }
}

#[derive(Clone, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Failed {
    pub error_code: ErrorCode,
}

impl Failed {
    fn parse(data: &[u8]) -> Result<Self, ParseError> {
        if data.len() != 2 && data[0] != ProvisioningPDU::FAILED {
            Err(ParseError::InvalidPDUFormat)
        } else {
            Ok(Self {
                error_code: ErrorCode::parse(data[1])?,
            })
        }
    }
    pub fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        xmit.push(ProvisioningPDU::FAILED)?;
        self.error_code.emit(xmit)?;
        Ok(())
    }
}

impl ProvisioningPDU {
    const INVITE: u8 = 0x00;
    const CAPABILITIES: u8 = 0x01;
    const START: u8 = 0x02;
    const PUBLIC_KEY: u8 = 0x03;
    const INPUT_COMPLETE: u8 = 0x04;
    const CONFIRMATION: u8 = 0x05;
    const RANDOM: u8 = 0x06;
    const DATA: u8 = 0x07;
    const COMPLETE: u8 = 0x08;
    const FAILED: u8 = 0x09;

    pub fn parse(data: &[u8]) -> Result<Self, ParseError> {
        if !data.is_empty() {
            match data[0] {
                Self::INVITE => Ok(Self::Invite(Invite::parse(data)?)),
                Self::CAPABILITIES => Ok(Self::Capabilities(Capabilities::parse(data)?)),
                Self::START => Ok(Self::Start(Start::parse(data)?)),
                Self::PUBLIC_KEY => Ok(Self::PublicKey(PublicKey::parse(data)?)),
                Self::INPUT_COMPLETE => Self::parse_provisioning_input_complete(data),
                Self::CONFIRMATION => Ok(Self::Confirmation(Confirmation::parse(data)?)),
                Self::RANDOM => Ok(Self::Random(Random::parse(data)?)),
                Self::DATA => Ok(Self::Data(Data::parse(data)?)),
                Self::COMPLETE => Self::parse_complete(data),
                Self::FAILED => Ok(Self::Failed(Failed::parse(data)?)),
                _ => Err(ParseError::InvalidPDUFormat),
            }
        } else {
            Err(ParseError::InvalidLength)
        }
    }

    pub fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        match self {
            ProvisioningPDU::Invite(invite) => invite.emit(xmit),
            ProvisioningPDU::Capabilities(capabilities) => capabilities.emit(xmit),
            ProvisioningPDU::Start(start) => start.emit(xmit),
            ProvisioningPDU::PublicKey(public_key) => public_key.emit(xmit),
            ProvisioningPDU::InputComplete => Ok(xmit.push(Self::INPUT_COMPLETE)?),
            ProvisioningPDU::Confirmation(confirmation) => confirmation.emit(xmit),
            ProvisioningPDU::Random(random) => random.emit(xmit),
            ProvisioningPDU::Data(data) => data.emit(xmit),
            ProvisioningPDU::Complete => Ok(xmit.push(Self::COMPLETE)?),
            ProvisioningPDU::Failed(failed) => failed.emit(xmit),
        }
    }

    fn parse_provisioning_input_complete(data: &[u8]) -> Result<Self, ParseError> {
        if data.len() == 1 && data[0] == Self::INPUT_COMPLETE {
            Ok(Self::InputComplete)
        } else {
            Err(ParseError::InvalidPDUFormat)
        }
    }

    fn parse_complete(data: &[u8]) -> Result<Self, ParseError> {
        if data.len() != 1 && data[0] != Self::COMPLETE {
            Err(ParseError::InvalidPDUFormat)
        } else {
            Ok(Self::Complete)
        }
    }
}

#[derive(Clone, Default, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Algorithm {
    #[default]
    P256,
}

impl Algorithm {
    pub fn parse(octet: u8) -> Result<Self, ParseError> {
        if octet == 0x00 {
            Ok(Self::P256)
        } else {
            Err(ParseError::InvalidValue)
        }
    }

    pub fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        match self {
            Algorithm::P256 => xmit.push(0x00)?,
        }

        Ok(())
    }
}

#[derive(Clone, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Algorithms(Vec<Algorithm, 16>);

impl Algorithms {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    fn push(&mut self, algo: Algorithm) -> Result<(), Algorithm> {
        self.0.push(algo)
    }

    pub fn parse(bits: u16) -> Result<Self, ParseError> {
        if bits & 0b1111111111111110 != 0 {
            return Err(ParseError::InvalidValue);
        }

        let mut algos = Algorithms::new();

        if bits & 0b1 == 1 {
            algos
                .push(Algorithm::P256)
                .map_err(|_| ParseError::InsufficientBuffer)?;
        }

        Ok(algos)
    }

    pub fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        let bits: Option<u16> = self
            .0
            .iter()
            .map(|e| {
                match e {
                    Algorithm::P256 => 0b0000000000000001, // room for growth
                }
            })
            .reduce(|accum, e| accum | e);

        let bits = bits.unwrap_or(0);

        Ok(xmit.extend_from_slice(&bits.to_be_bytes())?)
    }
}

impl Default for Algorithms {
    fn default() -> Self {
        let mut algos = Self::new();
        // infallible
        algos.push(Algorithm::P256).ok();
        algos
    }
}

#[derive(Clone, Default, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PublicKeyType {
    pub available: bool,
}

impl PublicKeyType {
    pub fn parse(bits: u8) -> Result<Self, ParseError> {
        if bits & 0b11111110 != 0 {
            Err(ParseError::InvalidValue)
        } else {
            Ok(Self {
                available: (bits & 0b1 == 1),
            })
        }
    }

    pub fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        if self.available {
            xmit.push(0b1)?
        } else {
            xmit.push(0b0)?
        }
        Ok(())
    }
}

#[derive(Clone, Default, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PublicKeySelected {
    #[default]
    NoPublicKey,
    OOBPublicKey,
}

impl PublicKeySelected {
    pub fn parse(octet: u8) -> Result<Self, ParseError> {
        match octet {
            0x00 => Ok(Self::NoPublicKey),
            0x01 => Ok(Self::OOBPublicKey),
            _ => Err(ParseError::InvalidValue),
        }
    }

    pub fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        match self {
            PublicKeySelected::NoPublicKey => xmit.push(0x00)?,
            PublicKeySelected::OOBPublicKey => xmit.push(0x01)?,
        }

        Ok(())
    }
}

#[derive(Copy, Clone, Default, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StaticOOBType {
    pub available: bool,
}

impl StaticOOBType {
    pub fn parse(bits: u8) -> Result<Self, ParseError> {
        if bits & 0b11111110 != 0 {
            Err(ParseError::InvalidValue)
        } else {
            Ok(Self {
                available: bits & 0b1 == 1,
            })
        }
    }

    pub fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        if self.available {
            xmit.push(0b1)?
        } else {
            xmit.push(0b0)?
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Default, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OOBSize {
    #[default]
    NotSupported,
    MaximumSize(u8 /* 1-8 decimal */),
}

impl OOBSize {
    pub fn parse(octet: u8) -> Result<Self, ParseError> {
        if octet == 0 {
            Ok(Self::NotSupported)
        } else if octet < 8 {
            Ok(Self::MaximumSize(octet))
        } else {
            Err(ParseError::InvalidValue)
        }
    }

    pub fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        match self {
            OOBSize::NotSupported => xmit.push(0)?,
            OOBSize::MaximumSize(size) => xmit.push(*size)?,
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OutputOOBAction {
    Blink = 0b0000000000000001,
    Beep = 0b0000000000000010,
    Vibrate = 0b0000000000000100,
    OutputNumeric = 0b0000000000001000,
    OutputAlphanumeric = 0b0000000000010000,
}

impl OutputOOBAction {
    pub fn parse(octet: u8) -> Result<Self, ParseError> {
        match octet {
            0x00 => Ok(Self::Blink),
            0x01 => Ok(Self::Beep),
            0x02 => Ok(Self::Vibrate),
            0x03 => Ok(Self::OutputNumeric),
            0x04 => Ok(Self::OutputAlphanumeric),
            _ => Err(ParseError::InvalidValue),
        }
    }

    pub fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        Ok(xmit.extend_from_slice(&(*self as u16).to_be_bytes())?)
    }
}

impl From<OutputOOBAction> for ParseError {
    fn from(_: OutputOOBAction) -> Self {
        Self::InsufficientBuffer
    }
}

#[derive(Clone, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OutputOOBActions(Vec<OutputOOBAction, 5>);

impl OutputOOBActions {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, action: OutputOOBAction) -> Result<(), OutputOOBAction> {
        self.0.push(action)
    }

    pub fn parse(bits: u16) -> Result<Self, ParseError> {
        if bits & 0b1111111111100000 != 0 {
            return Err(ParseError::InvalidValue);
        }

        let mut actions = OutputOOBActions::new();
        if bits & 0b00000001 != 0 {
            actions.push(OutputOOBAction::Blink)?;
        }

        if bits & 0b00000010 != 0 {
            actions.push(OutputOOBAction::Beep)?;
        }

        if bits & 0b00000100 != 0 {
            actions.push(OutputOOBAction::Vibrate)?;
        }

        if bits & 0b00001000 != 0 {
            actions.push(OutputOOBAction::OutputNumeric)?;
        }

        if bits & 0b00010000 != 0 {
            actions.push(OutputOOBAction::OutputAlphanumeric)?;
        }

        Ok(actions)
    }

    pub fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        let bits = self
            .0
            .iter()
            .map(|e| *e as u16)
            .reduce(|accum, e| accum | e);

        let bits = bits.unwrap_or(0);

        Ok(xmit.extend_from_slice(&bits.to_be_bytes())?)
    }
}

impl Default for OutputOOBActions {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Copy, Clone, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InputOOBAction {
    Push = 0b0000000000000001,
    Twist = 0b0000000000000010,
    InputNumeric = 0b0000000000000100,
    InputAlphanumeric = 0b0000000000001000,
}

impl InputOOBAction {
    pub fn parse(octet: u8) -> Result<Self, ParseError> {
        match octet {
            0x00 => Ok(Self::Push),
            0x01 => Ok(Self::Twist),
            0x02 => Ok(Self::InputNumeric),
            0x03 => Ok(Self::InputAlphanumeric),
            _ => Err(ParseError::InvalidValue),
        }
    }

    pub fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        Ok(xmit.extend_from_slice(&(*self as u16).to_be_bytes())?)
    }
}

impl From<InputOOBAction> for ParseError {
    fn from(_: InputOOBAction) -> Self {
        Self::InsufficientBuffer
    }
}

#[derive(Clone, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InputOOBActions(Vec<InputOOBAction, 4>);

impl InputOOBActions {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, action: InputOOBAction) -> Result<(), InputOOBAction> {
        self.0.push(action)
    }

    pub fn parse(bits: u16) -> Result<Self, ParseError> {
        if bits & 0b1111111111110000 != 0 {
            return Err(ParseError::InvalidValue);
        }

        let mut actions = InputOOBActions::new();
        if bits & 0b00000001 != 0 {
            actions.push(InputOOBAction::Push)?;
        }

        if bits & 0b00000010 != 0 {
            actions.push(InputOOBAction::Twist)?;
        }

        if bits & 0b00000100 != 0 {
            actions.push(InputOOBAction::InputNumeric)?;
        }

        if bits & 0b00001000 != 0 {
            actions.push(InputOOBAction::InputAlphanumeric)?;
        }

        Ok(actions)
    }

    pub fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        let bits = self
            .0
            .iter()
            .map(|e| *e as u16)
            .reduce(|accum, e| accum | e);

        let bits = bits.unwrap_or(0);

        Ok(xmit.extend_from_slice(&bits.to_be_bytes())?)
    }
}

impl Default for InputOOBActions {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OOBAction {
    #[default]
    None,
    Output(OutputOOBAction),
    Input(InputOOBAction),
}

impl OOBAction {
    pub fn parse(method: &AuthenticationMethod, octet: u8) -> Result<Self, ParseError> {
        match method {
            AuthenticationMethod::No | AuthenticationMethod::Static => {
                if octet != 0 {
                    Err(ParseError::InvalidValue)
                } else {
                    Ok(Self::None)
                }
            }
            AuthenticationMethod::Output => Ok(Self::Output(OutputOOBAction::parse(octet)?)),
            AuthenticationMethod::Input => Ok(Self::Input(InputOOBAction::parse(octet)?)),
        }
    }

    pub fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        match self {
            OOBAction::None => xmit.push(0x00)?,
            OOBAction::Output(action) => {
                action.emit(xmit)?;
            }
            OOBAction::Input(action) => {
                action.emit(xmit)?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Default, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AuthenticationMethod {
    #[default]
    No = 0x00,
    Static = 0x01,
    Output = 0x02,
    Input = 0x03,
}

impl AuthenticationMethod {
    pub fn parse(octet: u8) -> Result<Self, ParseError> {
        match octet {
            0x00 => Ok(Self::No),
            0x01 => Ok(Self::Static),
            0x02 => Ok(Self::Output),
            0x03 => Ok(Self::Input),
            _ => Err(ParseError::InvalidValue),
        }
    }

    pub fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        match self {
            AuthenticationMethod::No => xmit.push(0x00)?,
            AuthenticationMethod::Static => xmit.push(0x01)?,
            AuthenticationMethod::Output => xmit.push(0x02)?,
            AuthenticationMethod::Input => xmit.push(0x03)?,
        }
        Ok(())
    }
}

#[derive(Clone, Hash, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ErrorCode {
    Prohibited = 0x00,
    InvalidPDU = 0x01,
    InvalidFormat = 0x02,
    UnexpectedPDU = 0x03,
    ConfirmationFailed = 0x04,
    OutOfResources = 0x05,
    DecryptionFailed = 0x06,
    UnexpectedError = 0x07,
    CannotAssignAddresses = 0x08,
}

impl ErrorCode {
    pub fn parse(octet: u8) -> Result<Self, ParseError> {
        match octet {
            0x00 => Ok(Self::Prohibited),
            0x01 => Ok(Self::InvalidPDU),
            0x02 => Ok(Self::InvalidFormat),
            0x03 => Ok(Self::UnexpectedPDU),
            0x04 => Ok(Self::ConfirmationFailed),
            0x05 => Ok(Self::OutOfResources),
            0x06 => Ok(Self::DecryptionFailed),
            0x07 => Ok(Self::UnexpectedError),
            0x08 => Ok(Self::CannotAssignAddresses),
            _ => Err(ParseError::InvalidValue),
        }
    }
    pub fn emit<const N: usize>(&self, xmit: &mut Vec<u8, N>) -> Result<(), InsufficientBuffer> {
        match self {
            Self::Prohibited => xmit.push(0x00)?,
            Self::InvalidPDU => xmit.push(0x01)?,
            Self::InvalidFormat => xmit.push(0x02)?,
            Self::UnexpectedPDU => xmit.push(0x03)?,
            Self::ConfirmationFailed => xmit.push(0x04)?,
            Self::OutOfResources => xmit.push(0x05)?,
            Self::DecryptionFailed => xmit.push(0x06)?,
            Self::UnexpectedError => xmit.push(0x07)?,
            Self::CannotAssignAddresses => xmit.push(0x08)?,
        }
        Ok(())
    }
}
