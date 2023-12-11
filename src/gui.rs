use iced::alignment::Alignment;
use iced::theme::Theme;
use iced::widget::{text_input, row};
use iced::{window, Command, Application, executor, Subscription, subscription, Event};
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
    window_id: String,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    WindowOpened(Event),
}

impl Application for Searcher {
    type Message = Message;

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Searcher {input_value: "".to_string(), window_id: "123".to_string()}, Command::none())
    }

    fn title(&self) -> String {
        String::from("")
    }

    fn update(&mut self, message: Message) -> Command<Message>{
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
                Command::none()
            },
            Message::WindowOpened(Event::Window(window::Event::Focused)) => {
                text_input::focus(text_input::Id::new(self.window_id.clone()))

            },
            _ => Command::none()
        }
    }

    fn view(&self) -> Element<Message> {
        row![text_input("Search an app...", &self.input_value)
            .on_input(Message::InputChanged)
            .id(text_input::Id::new(self.window_id.clone()))
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

    type Executor = executor::Default;

    type Theme = iced::Theme;
    type Flags = ();
}
