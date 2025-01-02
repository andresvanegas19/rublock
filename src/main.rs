// filepath: /workspaces/rublock/src/main.rs
#[macro_use]
extern crate log;

mod encrypting {
    pub mod sha256;
}

fn main() {
    env_logger::init();
    info!("Starting the application");
}