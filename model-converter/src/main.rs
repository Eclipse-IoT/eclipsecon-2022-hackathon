use actix_web::{get, post, App, HttpResponse, HttpServer};
use btmesh_common::opcode::Opcode;
use btmesh_models::{
    generic::{
        battery::{GenericBatteryClient, GenericBatteryFlagsPresence, GenericBatteryMessage},
        onoff::{GenericOnOffClient, GenericOnOffServer},
    },
    sensor::{SensorClient, SensorMessage},
    Model,
};
use cloudevents::{Data, Event};
use sensor_model::*;
use serde_json::{json, Value};

#[post("/")]
async fn convert_event(mut event: Event) -> Event {
    println!("Received Event: {:?}", event);
    if let Some(Data::Json(data)) = event.data() {
        if let Ok(data) = serde_json::from_value(data.clone()) {
            let converted = convert_message(data).await;
            if let Some(converted) = converted {
                event.set_data("application/json", converted);
            }
        }
    }
    event
}

async fn convert_message(msg: RawMessage) -> Option<Value> {
    let (opcode, _) = Opcode::split(&msg.opcode[..]).unwrap();
    let parameters = &msg.parameters[..];
    match SensorClient::<MicrobitSensorConfig, 1, 1>::parse(opcode, parameters) {
        Ok(Some(SensorMessage::Status(status))) => {
            println!("Received sensor status {:?}", status);
            return Some(json!( {
                "state": {
                    "sensor": serde_json::to_value(&status.data).unwrap(),
                },
                "partial": false,
            }));
        }
        _ => {}
    }

    match GenericBatteryClient::parse(opcode, parameters) {
        Ok(Some(GenericBatteryMessage::Status(status))) => {
            println!("Received battery status {:?}", status);
            return Some(json!( {
                "state": {
                    "battery": {
                        "level": status.battery_level,
                        "flags": {
                            "presence": match status.flags.presence {
                                GenericBatteryFlagsPresence::NotPresent => "NotPresent",
                                GenericBatteryFlagsPresence::PresentRemovable => "PresentRemovable",
                                GenericBatteryFlagsPresence::PresentNotRemovable => "PresentNotRemovable",
                                GenericBatteryFlagsPresence::Unknown => "Unknown",
                            }
                        }
                    }
                },
                "partial": false,
            }));
        }
        _ => {}
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
            .service(convert_event)
    })
    .bind("0.0.0.0:8080")?
    .workers(1)
    .run()
    .await
}
