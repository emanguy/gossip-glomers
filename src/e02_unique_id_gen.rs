use log::*;
use maelstrom_rs::actor::Actor;
use maelstrom_rs::error::Error;
use maelstrom_rs::message::{Request, Response};
use serde_json::{Map, Number, Value};
use std::sync::atomic::{AtomicU64, Ordering};

struct UniqueIdPayload {
    unique_id: u64,
}

// Converts the response payload to the generic map type expected by maelstrom
impl From<UniqueIdPayload> for Map<String, Value> {
    fn from(value: UniqueIdPayload) -> Self {
        let mut map = Map::new();
        map.insert("id".to_string(), Value::from(Number::from(value.unique_id)));
        map
    }
}

pub struct UniqueIdActor {
    next_id: AtomicU64,
    num_actors: u64,
}

impl UniqueIdActor {
    pub fn new() -> UniqueIdActor {
        UniqueIdActor {
            next_id: AtomicU64::new(0),
            num_actors: 0,
        }
    }

    fn generate_uid(&self) -> u64 {
        self.next_id.fetch_add(self.num_actors, Ordering::SeqCst)
    }
}

impl Actor for UniqueIdActor {
    fn init(&mut self, node_id: &str, node_ids: Vec<String>) -> Result<(), Error> {
        let node_index = node_ids
            .iter()
            .position(|id| id == node_id)
            .expect("Our node id wasn't provided in the master list!")
            as u64;
        self.next_id.store(node_index, Ordering::SeqCst);
        self.num_actors = node_ids.len() as u64;

        info!(
            "Node {} initialized with index {} out of {} nodes",
            node_id, node_index, self.num_actors
        );
        Ok(())
    }

    fn receive(&mut self, request: &Request) -> Result<Vec<Response>, Error> {
        info!("Got a {} message", request.message_type);

        let response = match request.message_type.as_str() {
            "generate" => {
                let response_body = UniqueIdPayload {
                    unique_id: self.generate_uid(),
                };
                Response::new_from_request(request, response_body.into())
            }
            _ => {
                error!("Message type {} is unsupported", request.message_type);
                return Err(Error::NotSupported);
            }
        };

        Ok(vec![response])
    }
}
