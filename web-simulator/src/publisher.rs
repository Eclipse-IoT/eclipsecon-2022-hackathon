use btmesh_models::Message;
use sensor_model::RawMessage;

pub trait Publisher {
    fn send(&self, payload: String) -> anyhow::Result<()>;
}

pub trait PublisherExt: Publisher {
    fn publish<M: Message>(&self, msg: &M) -> anyhow::Result<()> {
        let mut opcode: heapless::Vec<u8, 16> = heapless::Vec::new();
        msg.opcode()
            .emit(&mut opcode)
            .map_err(|_| std::fmt::Error)?;

        let mut parameters: heapless::Vec<u8, 386> = heapless::Vec::new();
        msg.emit_parameters(&mut parameters)
            .map_err(|_| std::fmt::Error)?;
        let message = RawMessage {
            address: None,
            location: 0,
            opcode: opcode.to_vec(),
            parameters: parameters.to_vec(),
        };
        let data = serde_json::to_string(&message).map_err(|_| std::fmt::Error)?;

        self.send(data)
    }
}

impl<P: ?Sized + Publisher> PublisherExt for P {}
