use actix_web::{get, post, App, HttpResponse, HttpServer};
use btmesh_common::opcode::Opcode;
use btmesh_models::{
    generic::{
        battery::{GenericBatteryClient, GenericBatteryFlagsPresence, GenericBatteryMessage},
        onoff::{GenericOnOffMessage, GenericOnOffServer, Set as GenericOnOffSet},
    },
    sensor::SensorMessage,
    Message, Model,
};
use cloudevents::{Data, Event};
use sensor_model::*;
use serde_json::{json, Value};

#[post("/telemetry")]
async fn convert_telemetry(mut event: Event) -> Event {
    println!("Received Event: {:?}", event);
    if let Some(Data::Json(data)) = event.data() {
        if let Ok(data) = serde_json::from_value::<RawMessage>(data.clone()) {
            let converted = telemetry2json(data).await;
            if let Some(state) = converted {
                let output = json!({
                    "state": state,
                    "partial": true,
                });
                event.set_data("application/json", output);
            }
        }
    }
    event
}

#[post("/command")]
async fn convert_command(mut event: Event) -> Event {
    println!("Received Command: {:?}", event);
    if let Some(Data::Json(data)) = event.data() {
        if let Some(output) = json2command(data) {
            let output = serde_json::to_value(output).unwrap();
            event.set_data("application/json", output);
        }
    }
    event
}

fn json2command(data: &Value) -> Option<RawMessage> {
    if let Value::Object(data) = data {
        if let Some(Value::Object(state)) = data.get("button") {
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

async fn telemetry2json(msg: RawMessage) -> Option<Value> {
    let (opcode, _) = Opcode::split(&msg.opcode[..]).unwrap();
    let parameters = &msg.parameters[..];
    let location = msg.location;

    if let Ok(Some(GenericOnOffMessage::Set(set))) = GenericOnOffServer::parse(&opcode, parameters)
    {
        return Some(json!({ "button": {"on": set.on_off == 1, "location": location }}));
    }

    if let Ok(Some(GenericOnOffMessage::SetUnacknowledged(set))) =
        GenericOnOffServer::parse(&opcode, parameters)
    {
        return Some(json!({ "button": {"on": set.on_off == 1, "location": location }}));
    }

    if let Ok(Some(SensorMessage::Status(status))) = SensorClient::parse(&opcode, parameters) {
        println!("Received sensor status {:?}", status);
        return Some(json!( {
            "sensor": {
                "payload": serde_json::to_value(&status.data).unwrap(),
                "location": location,
            }
        }));
    }

    if let Ok(Some(GenericBatteryMessage::Status(status))) =
        GenericBatteryClient::parse(&opcode, parameters)
    {
        println!("Received battery status {:?}", status);
        return Some(json!( {
            "battery": {
                "level": status.battery_level,
                "flags": {
                    "presence": match status.flags.presence {
                        GenericBatteryFlagsPresence::NotPresent => "NotPresent",
                        GenericBatteryFlagsPresence::PresentRemovable => "PresentRemovable",
                        GenericBatteryFlagsPresence::PresentNotRemovable => "PresentNotRemovable",
                        GenericBatteryFlagsPresence::Unknown => "Unknown",
                    }
                },
                "location": location
            },
        }));
    }

    None
}

#[get("/healthz")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .service(health)
            .service(convert_telemetry)
            .service(convert_command)
    })
    .bind("0.0.0.0:8080")?
    .workers(1)
    .run()
    .await
}
