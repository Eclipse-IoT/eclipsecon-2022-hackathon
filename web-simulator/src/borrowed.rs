use btmesh_models::{
    generic::onoff::{GenericOnOffMessage, Set as GenericOnOffSet},
    Message,
};
use sensor_model::RawMessage;
use serde_json::Value;

// Converts JSON message to BLE mesh message
// TODO: This should eventually be done by the model-converter, but support
// calling command hooks in drogue-cloud is not yet available.
pub fn json2command(data: &Value) -> Option<RawMessage> {
    if let Value::Object(data) = data {
        if let Some(Value::Object(state)) = data.get("display") {
            let location = state["location"].as_u64().unwrap_or(0);
            let on = state["on"].as_bool().unwrap_or(false);
            let set = GenericOnOffSet {
                on_off: if on { 1 } else { 0 },
                tid: 0,
                transition_time: None,
                delay: None,
            };
            let msg = GenericOnOffMessage::Set(set);

            let mut opcode: heapless::Vec<u8, 16> = heapless::Vec::new();
            msg.opcode().emit(&mut opcode).unwrap();

            let mut parameters: heapless::Vec<u8, 386> = heapless::Vec::new();
            msg.emit_parameters(&mut parameters).unwrap();
            let message = RawMessage {
                location: location as u16,
                opcode: opcode.to_vec(),
                parameters: parameters.to_vec(),
            };
            return Some(message);
        }
    }
    None
}
