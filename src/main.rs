mod config;
mod connect;
mod login;
mod proxmox;

use crate::config::{AuthMethod, Cluster, Config, User};
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
    ($handle:ident, $svg:expr) => {
        static $handle: ::std::sync::LazyLock<::iced::widget::svg::Handle> =
            ::std::sync::LazyLock::new(|| {
                ::iced::widget::svg::Handle::from_memory(include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/assets/",
                    $svg
                )))
            });
    };
}

const CONFIG_PATH: &str = "./config.yaml";

fn main() -> iced::Result {
    iced::application(State::new, State::update, State::view)
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
struct State {
    config: Config,
    screen: Screen,
}

#[derive(Debug)]
enum Screen {
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
    pub fn new() -> Self {
        // TODO: Add error handling
        // let config = Config::load_file(CONFIG_PATH).expect("Config error handling not implemented");
        let config = Config {
            default_cluster: Some(0),
            clusters: vec![Cluster {
                name: "Cluster1".to_owned(),
                hosts: vec![],
                default_user: Some(0),
                users: vec![
                    User {
                        name: "User1".to_owned(),
                        auth_method: AuthMethod::Password,
                    },
                    User {
                        name: "User2".to_owned(),
                        auth_method: AuthMethod::ApiToken("PROXMOX-API-TOKEN".to_owned()),
                    },
                ],
            }],
            viewer_args: vec![],
        };

        let screen = Screen::Login(login::State::new(&config));

        Self { config, screen }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let screen_sub = if let Screen::Connect(state) = &self.screen {
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
                if let Screen::Login(state) = &mut self.screen {
                    match state.update(message, &mut self.config.clusters) {
                        login::Action::Login(auth, user) => {
                            let (state, task) = connect::State::new(auth, user);
                            self.screen = Screen::Connect(state);
                            task.map(Message::Connect)
                        }
                        login::Action::Run(task) => task.map(Message::Login),
                        login::Action::SaveConfig => {
                            // TODO: save config
                            Task::none()
                        }
                        login::Action::None => Task::none()
                    }
                } else {
                    Task::none()
                }
            }
            Message::Connect(message) => {
                if let Screen::Connect(state) = &mut self.screen {
                    match state.update(message, &mut self.config) {
                        connect::Action::Logout => {
                            self.screen = Screen::Login(login::State::new(&self.config));
                            Task::none()
                        }
                        connect::Action::Run(task) => task.map(Message::Connect),
                        connect::Action::None => Task::none()
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
        let screen = match &self.screen {
            Screen::Login(state) => state.view(&self.config.clusters).map(Message::Login),
            Screen::Connect(state) => state.view(&self.config).map(Message::Connect),
        };

        if cfg!(feature = "dev_mode") {
            screen.explain(iced::color!(0xcc_cc_cc))
        } else {
            screen
        }
    }
}
