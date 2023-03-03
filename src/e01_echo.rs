use log::*;
use maelstrom_rs::actor::Actor;
use maelstrom_rs::error::Error;
use maelstrom_rs::message::{Request, Response};
use serde_json::{Map, Value};

pub struct EchoActor {}

impl EchoActor {
    pub fn new() -> EchoActor {
        EchoActor {}
    }
}

impl Actor for EchoActor {
    fn init(&mut self, node_id: &str, _node_ids: Vec<String>) -> Result<(), Error> {
        // Do nothing
        info!("Node {} initialized.", node_id);
        Ok(())
    }

    fn receive(&mut self, request: &Request) -> Result<Vec<Response>, Error> {
        info!(
            "Got {} request from node {}",
            request.message_type, request.source
        );

        let response = match request.message_type.as_str() {
            "echo" => {
                let echo_text = request.body.get("echo").unwrap().as_str().unwrap();
                let mut response_body = Map::new();
                response_body.insert("echo".to_string(), Value::from(String::from(echo_text)));

                Response::new_from_request(request, response_body)
            }
            _ => {
                error!("Message type {} is unsupported.", request.message_type);
                return Err(Error::NotSupported);
            }
        };

        Ok(vec![response])
    }
}
