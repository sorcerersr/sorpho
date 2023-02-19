use config::Config;

mod config;
mod gui;

pub fn main() {
    env_logger::init();

    let config = Config::init();
    println!("config to use {:?}", config);

    gui::run();
}
