mod apps;
pub mod controllers;
pub mod entities;

fn main() {
    env_logger::init();
    apps::run().expect("Program failed");
}
