use log::*;
use maelstrom_rs::runtime::Runtime;

mod e01_echo;
mod e02_unique_id_gen;

fn main() {
    if env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .try_init()
        .is_err()
    {
        println!("Could not initialize logger!");
    }

    info!("Nodes spinning up!");
    // let node = Box::new(e01_echo::EchoActor::new());
    let node = Box::new(e02_unique_id_gen::UniqueIdActor::new());
    Runtime::new(node).start();
}
