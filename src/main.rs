mod apps;
pub mod controllers;
pub mod entities;
pub mod utils;

fn main() {
    env_logger::init();
    apps::run().expect("Program failed");
}
