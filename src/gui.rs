use iced::alignment::Alignment;
use iced::keyboard::KeyCode;
use iced::event::{self, Event};
use iced::theme::Theme;
use iced::widget::{row, text_input};
use iced::{executor, keyboard, window, subscription, Command, Event as AppEvent, Subscription, Application};
use iced::{Element, Length, Settings};
use std::process;

use crate::filter::{filter_entries, get_desktop_entries, FilteredEntry};

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
    desktop_entries: Vec<FilteredEntry>,
    filtered_desktop_entries: Vec<FilteredEntry>,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    WindowOpened,
    Executed,
    Exited
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
                desktop_entries: get_desktop_entries(),
                filtered_desktop_entries: Vec::default(),
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
                self.input_value = value.clone();
                self.filtered_desktop_entries =
                    filter_entries(self.desktop_entries.clone(), &value, 0);
                println!("{:?}", self.filtered_desktop_entries);
                Command::none()
            }
            Message::WindowOpened => {
                text_input::focus(self.input_id.clone())
            }
            Message::Executed => {
                match self.filtered_desktop_entries.first() {
                    Some(val) => {
                            process::Command::new("sh")
                            .arg("-c")
                            .arg(val.exec_path.clone())
                            .spawn()
                            .expect("Can not execute");
                        window::close()
                    }
                    _ => Command::none()
                }
            }
            Message::Exited => {
                window::close()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        row![text_input("Search an app...", &self.input_value)
            .on_input(Message::InputChanged)
            .on_submit(Message::Executed)
            .id(self.input_id.clone())
            .width(Length::Fill)
            .size(30)]
        .align_items(Alignment::Center)
        .height(Length::Shrink)
        .width(Length::Shrink)
        .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        subscription::events_with(|event, status| match (event, status) {
            (
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: KeyCode::Enter,
                    modifiers: _
                }),
                event::Status::Ignored,
            ) => Some(Message::Executed),
            (
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key_code: KeyCode::Escape,
                    modifiers: _
                }),
                event::Status::Ignored,
            ) => Some(Message::Exited),
            (
                AppEvent::Window(window::Event::Focused), 
                event::Status::Ignored,
            ) => Some(Message::WindowOpened),
            _ => None,
        })
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
