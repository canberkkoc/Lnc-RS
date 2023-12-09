use iced::alignment::{self, Alignment};
use iced::theme::Theme;
use iced::widget::{row, text_input};
use iced::{window, Element, Length, Sandbox, Settings};

pub fn gui_run() -> iced::Result {
    let settings: Settings<()> = iced::settings::Settings {
        window: window::Settings {
            size: (300, 40),
            resizable: (false),
            decorations: (false),
            ..Default::default()
        },
        ..Default::default()
    };
    Styling::run(settings)
}

#[derive(Default)]
struct Styling {
    input_value: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
}

impl Sandbox for Styling {
    type Message = Message;

    fn new() -> Self {
        Styling::default()
    }

    fn title(&self) -> String {
        String::from("")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::InputChanged(value) => self.input_value = value,
        }
    }

    fn view(&self) -> Element<Message> {
        row![text_input("Search an app...", &self.input_value)
            .on_input(Message::InputChanged)
            .size(20)]
        .align_items(Alignment::Center)
        .height(Length::Shrink)
        .width(Length::Shrink)
        .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
