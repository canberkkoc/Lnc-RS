use iced::alignment::Alignment;
use iced::theme::Theme;
use iced::widget::{row, text_input};
use iced::{executor, subscription, window, Application, Command, Event, Subscription};
use iced::{Element, Length, Settings};

pub fn gui_run() -> iced::Result {
    let settings: Settings<()> = iced::settings::Settings {
        window: window::Settings {
            size: (300, 40),
            resizable: (false),
            decorations: (false),
            position: window::Position::Centered,
            ..Default::default()
        },
        ..Default::default()
    };
    Searcher::run(settings)
}
#[derive(Clone)]
struct Searcher {
    input_value: String,
    input_id: text_input::Id,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    WindowOpened(Event),
}

impl Application for Searcher {
    type Message = Message;
    type Executor = executor::Default;
    type Theme = iced::Theme;

    type Flags = ();
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Searcher {
                input_value: "".to_string(),
                input_id: text_input::Id::new("1"),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
                Command::none()
            }
            Message::WindowOpened(Event::Window(window::Event::Focused)) => {
                text_input::focus(self.input_id.clone())
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        row![text_input("Search an app...", &self.input_value)
            .on_input(Message::InputChanged)
            .id(self.input_id.clone())
            .size(20)]
        .align_items(Alignment::Center)
        .height(Length::Shrink)
        .width(Length::Shrink)
        .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        subscription::events().map(Message::WindowOpened)
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
