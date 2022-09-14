#![cfg_attr(not(feature = "std"), no_std)]
use btmesh_common::{InsufficientBuffer, ParseError};
use btmesh_models::sensor::{
    CadenceDescriptor, PropertyId, SensorClient as SC, SensorConfig, SensorData, SensorDescriptor,
    SensorMessage as SM, SensorSetupConfig, SensorSetupServer, SettingDescriptor,
};
use heapless::Vec;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MicrobitSensorConfig;

pub type SensorServer = SensorSetupServer<MicrobitSensorConfig, 3, 1>;
pub type SensorClient = SC<MicrobitSensorConfig, 3, 1>;
pub type SensorMessage = SM<MicrobitSensorConfig, 3, 1>;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct SensorPayload {
    pub temperature: i8,
    pub acceleration: Acceleration,
    pub noise: u8,
}

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Acceleration {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl Default for Acceleration {
    fn default() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

const PROP_TEMP: PropertyId = PropertyId(0x4F);
const PROP_XL: PropertyId = PropertyId(0x4242);
const PROP_NOISE: PropertyId = PropertyId(0x79);

impl Default for SensorPayload {
    fn default() -> Self {
        Self {
            temperature: 0,
            noise: 0,
            acceleration: Acceleration::default(),
        }
    }
}

impl SensorData for SensorPayload {
    fn decode(&mut self, id: PropertyId, params: &[u8]) -> Result<(), ParseError> {
        if id == PROP_TEMP {
            self.temperature = params[0] as i8;
            Ok(())
        } else if id == PROP_XL {
            self.acceleration = Acceleration::default();
            self.acceleration.x = i16::from_le_bytes([params[0], params[1]]);
            self.acceleration.y = i16::from_le_bytes([params[2], params[3]]);
            self.acceleration.z = i16::from_le_bytes([params[4], params[5]]);
            Ok(())
        } else if id == PROP_NOISE {
            self.noise = params[0];
            Ok(())
        } else {
            Err(ParseError::InvalidValue)
        }
    }

    fn encode<const N: usize>(
        &self,
        property: PropertyId,
        xmit: &mut Vec<u8, N>,
    ) -> Result<(), InsufficientBuffer> {
        if property == PROP_TEMP {
            xmit.extend_from_slice(&self.temperature.to_le_bytes())
                .map_err(|_| InsufficientBuffer)?;
        } else if property == PROP_XL {
            xmit.extend_from_slice(&self.acceleration.x.to_le_bytes())
                .map_err(|_| InsufficientBuffer)?;
            xmit.extend_from_slice(&self.acceleration.y.to_le_bytes())
                .map_err(|_| InsufficientBuffer)?;
            xmit.extend_from_slice(&self.acceleration.z.to_le_bytes())
                .map_err(|_| InsufficientBuffer)?;
        } else if property == PROP_NOISE {
            xmit.extend_from_slice(&self.noise.to_le_bytes())
                .map_err(|_| InsufficientBuffer)?;
        }
        Ok(())
    }
}

impl SensorConfig for MicrobitSensorConfig {
    type Data = SensorPayload;

    const DESCRIPTORS: &'static [SensorDescriptor] = &[
        SensorDescriptor::new(PROP_TEMP, 1),
        SensorDescriptor::new(PROP_XL, 6),
        SensorDescriptor::new(PROP_NOISE, 1),
    ];
}

impl SensorSetupConfig for MicrobitSensorConfig {
    const CADENCE_DESCRIPTORS: &'static [CadenceDescriptor] = &[];
    const SETTING_DESCRIPTORS: &'static [SettingDescriptor] = &[];
}

/// RawMessage contains the opcode and message payload
#[cfg(feature = "std")]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RawMessage {
    pub address: u16,
    pub location: u16,
    pub opcode: std::vec::Vec<u8>,
    pub parameters: std::vec::Vec<u8>,
}

#[cfg(feature = "std")]
impl btmesh_models::Message for RawMessage {
    fn opcode(&self) -> btmesh_common::opcode::Opcode {
        let (opcode, _) = btmesh_common::opcode::Opcode::split(&self.opcode[..]).unwrap();
        opcode
    }

    fn emit_parameters<const N: usize>(
        &self,
        parameters: &mut heapless::Vec<u8, N>,
    ) -> Result<(), InsufficientBuffer> {
        parameters
            .extend_from_slice(&self.parameters[..])
            .map_err(|_| InsufficientBuffer)
    }
}
