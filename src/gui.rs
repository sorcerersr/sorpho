use std::env;

use iced::widget::{column, container, row, text};
use iced::{Color, Element, Length, Sandbox, Settings, Theme};

use crate::config::Config;

pub fn run() -> iced::Result {
    Sorpho::run(Settings::default())
}

#[derive(Default)]
struct Sorpho {
    debug: bool,
    config: Config,
}

#[derive(Debug, Clone)]
enum Message {}

impl Sandbox for Sorpho {
    type Message = Message;

    fn new() -> Self {
        let config = Config::init().unwrap();
        log::info!("config to use {:?}", config);

        let args: Vec<String> = env::args().collect();
        let debug = args.contains(&"debuglayout".to_owned());
        log::info!("args = {:#?}", args);
        Sorpho { debug, config }
    }

    fn title(&self) -> String {
        String::from("Sorpho")
    }

    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<Message> {
        let title = text("top text").width(Length::Fill).size(50);

        let toptext = text("left text").width(Length::Fill).size(50);
        let bottomtext = text("right text").width(Length::Fill).size(50);

        let content: Element<_> = column![title, row![toptext, bottomtext]].into();

        container(if self.debug {
            content.explain(Color::BLACK)
        } else {
            content
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
