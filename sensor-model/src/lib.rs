#![cfg_attr(not(feature = "std"), no_std)]
use btmesh_models::{
    sensor::{
        CadenceDescriptor, PropertyId, SensorConfig, SensorData, SensorDescriptor,
        SensorSetupConfig, SettingDescriptor,
    },
};
use btmesh_common::{InsufficientBuffer, ParseError,};
use heapless::Vec;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MicrobitSensorConfig;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SensorPayload {
    pub temperature: i8,
}

impl Default for SensorPayload {
    fn default() -> Self {
        Self { temperature: 0 }
    }
}

impl SensorData for SensorPayload {
    fn decode(&mut self, id: PropertyId, params: &[u8]) -> Result<(), ParseError> {
        if id.0 == 0x4F {
            self.temperature = params[0] as i8;
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
        if property == PropertyId(0x4F) {
            xmit.extend_from_slice(&self.temperature.to_le_bytes())
                .map_err(|_| InsufficientBuffer)?;
        }
        Ok(())
    }
}

impl SensorConfig for MicrobitSensorConfig {
    type Data = SensorPayload;

    const DESCRIPTORS: &'static [SensorDescriptor] = &[SensorDescriptor::new(PropertyId(0x4F), 1)];
}

impl SensorSetupConfig for MicrobitSensorConfig {
    const CADENCE_DESCRIPTORS: &'static [CadenceDescriptor] = &[];
    const SETTING_DESCRIPTORS: &'static [SettingDescriptor] = &[];
}

/// RawMessage contains the opcode and message payload
#[cfg(feature = "std")]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct RawMessage {
    opcode: std::vec::Vec<u8>,
    message: std::vec::Vec<u8>,
}
