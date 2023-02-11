use iced::widget::{column, container, text};
use iced::{alignment, Color, Element, Length, Sandbox, Settings, Theme};

pub fn main() -> iced::Result {
    Sorpho::run(Settings::default())
}

#[derive(Default)]
struct Sorpho {}

#[derive(Debug, Clone)]
enum Message {}

impl Sandbox for Sorpho {
    type Message = Message;

    fn new() -> Self {
        Sorpho::default()
    }

    fn title(&self) -> String {
        String::from("Sorpho")
    }

    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<Message> {
        let title = text("Sorpho")
            .width(Length::Fill)
            .size(100)
            .style(Color::from([0.5, 0.5, 0.5]))
            .horizontal_alignment(alignment::Horizontal::Center);

        let content = column![title].spacing(20).max_width(800);

        container(content)
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
