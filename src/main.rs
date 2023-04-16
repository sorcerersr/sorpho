use config::Config;

mod config;
mod gui;

pub fn main() {
    env_logger::init();
    gui::run();
}
