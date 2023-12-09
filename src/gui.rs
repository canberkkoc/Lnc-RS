use iced::theme::Theme;
use iced::widget::{column, container, row, text_input};
use iced::{Alignment, Element, Length, Sandbox, Settings};

pub fn gui_run() -> iced::Result {
    Styling::run(Settings::default())
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

        let text_input: Element <_> = text_input("Search an app...", &self.input_value)
            .on_input(Message::InputChanged)
            .padding(10)
            .size(20)
            .into();
        let content = column![
            row![text_input]
                .align_items(Alignment::Center)
                .width(Length::Shrink),
        ]
        .spacing(20)
        .padding(20)
        .height(Length::Fixed(150.0))
        .width(Length::Fixed(150.0))
        .max_width(400);

        container(content)
            .center_x()
            .center_y()
            .padding(10)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
