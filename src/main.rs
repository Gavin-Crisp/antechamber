mod connect;
mod login;
mod proxmox;

use iced::{
    event::{self, listen_with, Status}, keyboard::{self, key::Named, Key}, widget::operation,
    window::{Level, Settings},
    Element,
    Subscription,
    Task,
};

#[cfg(all(feature = "dev_mode", not(debug_assertions)))]
compile_error!("Release build should not include debug features");

#[macro_export]
macro_rules! include_svg {
    ($handle:ident, $svg:expr) => {static $handle: ::std::sync::LazyLock<::iced::widget::svg::Handle> = ::std::sync::LazyLock::new(|| ::iced::widget::svg::Handle::from_memory(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $svg))));}
}

fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .subscription(State::subscription)
        .title("Antechamber")
        .window(Settings {
            // Not strictly needed for intended use case, but I'll probably set one eventually
            icon: None,
            fullscreen: !cfg!(feature = "dev_mode"),
            minimizable: false,
            level: if cfg!(feature = "dev_mode") {
                Level::Normal
            } else {
                Level::AlwaysOnTop
            },
            decorations: true,
            ..Settings::default()
        })
        .run()
}

#[derive(Debug)]
enum State {
    Login(login::State),
    Connect(connect::State),
}

#[derive(Clone, Debug)]
enum Message {
    Login(login::Message),
    Connect(connect::Message),
    FocusNext,
    FocusPrev,
}

impl State {
    pub fn subscription(&self) -> Subscription<Message> {
        let screen_sub = if let Self::Connect(state) = self {
            Some(state.subscription().map(Message::Connect))
        } else {
            None
        };

        let focus_sub = listen_with(|event, status, _id| {
            if status == Status::Captured {
                return None;
            }

            let event::Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::Tab),
                modifiers,
                ..
            }) = event
            else {
                return None;
            };

            if modifiers.shift() {
                Some(Message::FocusPrev)
            } else {
                Some(Message::FocusNext)
            }
        });

        if let Some(screen_sub) = screen_sub {
            Subscription::batch([screen_sub, focus_sub])
        } else {
            focus_sub
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Login(message) => {
                if let Self::Login(state) = self {
                    match state.update(message) {
                        login::Action::Login(auth) => {
                            let (state, task) = connect::State::new(auth);
                            *self = Self::Connect(state);
                            task.map(Message::Connect)
                        }
                        login::Action::Run(task) => task.map(Message::Login),
                    }
                } else {
                    Task::none()
                }
            }
            Message::Connect(message) => {
                if let Self::Connect(state) = self {
                    match state.update(message) {
                        connect::Action::Logout => {
                            *self = Self::Login(login::State::default());
                            Task::none()
                        }
                        connect::Action::Run(task) => task.map(Message::Connect),
                    }
                } else {
                    Task::none()
                }
            }
            Message::FocusNext => operation::focus_next(),
            Message::FocusPrev => operation::focus_previous(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let screen = match self {
            Self::Login(state) => state.view().map(Message::Login),
            Self::Connect(state) => state.view().map(Message::Connect),
        };

        if cfg!(feature = "dev_mode") {
            screen.explain(iced::color!(0xcc_cc_cc))
        } else {
            screen
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Login(login::State::default())
    }
}
