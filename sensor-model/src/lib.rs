#![no_std]
use btmesh_models::{
    sensor::{
        CadenceDescriptor, PropertyId, SensorConfig, SensorData, SensorDescriptor,
        SensorSetupConfig, SettingDescriptor,
    },
    InsufficientBuffer, ParseError,
};
use heapless::Vec;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MicrobitSensorConfig;

#[derive(Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SensorPayload;

impl SensorData for SensorPayload {
    fn decode(&mut self, _: PropertyId, _: &[u8]) -> Result<(), ParseError> {
        todo!()
    }

    fn encode<const N: usize>(
        &self,
        _property: PropertyId,
        _xmit: &mut Vec<u8, N>,
    ) -> Result<(), InsufficientBuffer> {
        todo!()
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
