use crate::{
    config::{AuthMethod, Config},
    include_svg,
    modal::modal,
    proxmox::Auth,
};
use iced::{
    alignment::Horizontal, color, mouse::Interaction, widget::{
        button, center, column, container, mouse_area, operation, pick_list, row, stack, svg, text,
        text_input, Svg,
    },
    Element,
    Fill,
    Shrink,
    Task,
};

include_svg!(OPEN_EYE, "lucide/eye.svg");
include_svg!(CLOSED_EYE, "lucide/eye-off.svg");
include_svg!(ADD_USER, "lucide/user-plus.svg");

#[derive(Debug)]
pub struct State {
    modal: Option<user_modal::State>,
    cluster: Option<usize>,
    user: Option<usize>,
    password: Option<Password>,
}

#[derive(Debug)]
struct Password {
    text: String,
    secure: bool,
    error: bool,
}

#[derive(Clone, Debug)]
pub enum Message {
    SelectCluster(usize),
    SelectUser(usize),
    ShowModal,
    Modal(user_modal::Message),
    Password(String),
    ShowPassword,
    HidePassword,
    SubmitPassword,
    SubmitApi,
    Login(Auth),
}

#[derive(Debug)]
pub enum Action {
    Login {
        auth: Auth,
        cluster: usize,
        user: usize,
    },
    Run(Task<Message>),
    SaveConfig,
    None,
}

impl State {
    const PASSWORD_ID: &str = "password";

    pub fn new(config: &Config, user: Option<usize>) -> Self {
        let user = user.or(config.default_user);

        Self {
            modal: None,
            cluster: config.default_cluster,
            user,
            password: user.and_then(|idx| match config.users[idx].auth_method {
                AuthMethod::Password => Some(Password {
                    text: String::new(),
                    secure: true,
                    error: false,
                }),
                AuthMethod::ApiToken(_) => None,
            }),
        }
    }

    pub fn update(&mut self, message: Message, config: &mut Config) -> Action {
        match message {
            Message::SelectCluster(cluster) => {
                if self.cluster.is_none_or(|current| current != cluster) {
                    self.cluster = Some(cluster);
                }
                Action::None
            }
            Message::SelectUser(new) => {
                if self.user.is_none_or(|current| current != new) {
                    self.select_user(config, new);
                }
                Action::Run(operation::focus(Self::PASSWORD_ID))
            }
            Message::ShowModal => {
                let (state, task) = user_modal::State::new();
                self.modal = Some(state);
                Action::Run(task.map(Message::Modal))
            }
            Message::Modal(message) => {
                if let Some(state) = &mut self.modal {
                    match state.update(message) {
                        user_modal::Action::Add(user) => {
                            self.modal = None;
                            config.users.push(user);
                            self.select_user(config, config.users.len() - 1);

                            return Action::SaveConfig;
                        }
                        user_modal::Action::Close => self.modal = None,
                        user_modal::Action::None => {}
                    }
                }
                Action::None
            }
            Message::Password(text) => {
                if let Some(password) = &mut self.password {
                    password.text = text;
                }
                Action::None
            }
            Message::ShowPassword => {
                if let Some(password) = &mut self.password {
                    password.secure = false;
                }
                Action::None
            }
            Message::HidePassword => {
                if let Some(password) = &mut self.password {
                    password.secure = true;
                }
                Action::None
            }
            Message::SubmitPassword => {
                if let Some(password) = &mut self.password {
                    if self.user.is_some() && !password.text.is_empty() {
                        // TODO: replace with password login
                        Action::Run(Task::done(Message::Login(Auth {
                            csrf: String::new(),
                            ticket: String::new(),
                        })))
                    } else {
                        password.error = true;
                        Action::None
                    }
                } else {
                    Action::None
                }
            }
            Message::SubmitApi => {
                if self.user.is_some() {
                    // TODO: replace with api login
                    Action::Run(Task::done(Message::Login(Auth {
                        ticket: String::new(),
                        csrf: String::new(),
                    })))
                } else {
                    Action::None
                }
            }
            Message::Login(auth) => {
                if let Some(cluster) = self.cluster
                    && let Some(user) = self.user
                {
                    Action::Login {
                        auth,
                        cluster,
                        user,
                    }
                } else {
                    Action::None
                }
            }
        }
    }

    fn select_user(&mut self, config: &Config, user: usize) {
        self.user = Some(user);
        self.password = match config.users[user].auth_method {
            AuthMethod::Password => Some(Password {
                text: String::new(),
                secure: true,
                error: false,
            }),
            AuthMethod::ApiToken(_) => None,
        };
    }

    pub fn view<'a>(&'a self, config: &'a Config) -> Element<'a, Message> {
        let cluster_select = pick_list(
            config.clusters.as_slice(),
            self.cluster.map(|idx| &config.clusters[idx]),
            |cluster| {
                Message::SelectCluster(
                    config
                        .clusters
                        .iter()
                        .position(|c| c.name == cluster.name)
                        .expect("cluster is in clusters"),
                )
            },
        )
        .placeholder("Select cluster");

        let user_select = pick_list(
            config.users.as_slice(),
            self.user.map(|idx| &config.users[idx]),
            |user| {
                Message::SelectUser(
                    config
                        .users
                        .iter()
                        .position(|u| u.name == user.name)
                        .expect("user is in users"),
                )
            },
        )
        .placeholder("Select user");

        let add_user = button(svg(ADD_USER.clone()))
            .width(Shrink)
            .on_press(Message::ShowModal);

        let user = row![user_select, add_user];

        let auth: Option<Element<Message>> = self.user.map(|_| {
            self.password.as_ref().map_or_else(
                || button("Login").on_press(Message::SubmitApi).into(),
                |p| {
                    let password_input = text_input("Password", &p.text)
                        .on_input(Message::Password)
                        .on_submit(Message::SubmitPassword)
                        .secure(p.secure)
                        .id(Self::PASSWORD_ID);

                    let eye_svg: Svg = svg(if p.secure {
                        OPEN_EYE.clone()
                    } else {
                        CLOSED_EYE.clone()
                    });
                    let show_button =
                        mouse_area(container(eye_svg).center_x(35).center_y(Fill).padding(5))
                            .on_press(Message::ShowPassword)
                            .on_release(Message::HidePassword)
                            .interaction(Interaction::Pointer);

                    let error_message = if p.error {
                        Some(container(
                            text("Empty password is not valid").color(color!(0xff_00_00)),
                        ))
                    } else {
                        None
                    };

                    column![
                        row![password_input, show_button].height(Shrink),
                        error_message
                    ]
                    .into()
                },
            )
        });

        let input_box = column![cluster_select, user, auth]
            .padding(10)
            .spacing(10)
            .align_x(Horizontal::Center)
            .width(300)
            .padding(10);

        stack![
            center(input_box),
            self.modal.as_ref().map(|state| modal(
                state.view().map(Message::Modal),
                Message::Modal(user_modal::Message::Close)
            )),
        ]
        .into()
    }
}

mod user_modal {
    use crate::config::{AuthMethod, User};
    use iced::{
        widget::{button, column, container, operation, row, text_input}, Center, Element,
        Task,
    };
    use std::mem;

    #[derive(Clone, Debug)]
    pub struct State {
        user: User,
    }

    #[derive(Clone, Debug)]
    pub enum Message {
        DisplayName(String),
        Username(String),
        Password,
        Api,
        Token(String),
        Close,
        Submit,
    }

    pub enum Action {
        Add(User),
        Close,
        None,
    }

    impl State {
        const DISPLAY_NAME_ID: &str = "display_name";

        pub fn new() -> (Self, Task<Message>) {
            (
                Self {
                    user: User::default(),
                },
                operation::focus(Self::DISPLAY_NAME_ID),
            )
        }

        pub fn update(&mut self, message: Message) -> Action {
            match message {
                Message::DisplayName(display_name) => {
                    self.user.display_name = display_name;
                    Action::None
                }
                Message::Username(name) => {
                    self.user.name = name;
                    Action::None
                }
                Message::Password => {
                    self.user.auth_method = AuthMethod::Password;
                    Action::None
                }
                Message::Api => {
                    self.user.auth_method = AuthMethod::ApiToken(String::new());
                    Action::None
                }
                Message::Token(token) => {
                    if let AuthMethod::ApiToken(curr_token) = &mut self.user.auth_method {
                        *curr_token = token;
                    }
                    Action::None
                }
                Message::Close => Action::Close,
                Message::Submit => {
                    // TODO: give feedback on invalid submission
                    if self.is_valid() {
                        Action::Add(mem::take(&mut self.user))
                    } else {
                        Action::None
                    }
                }
            }
        }

        pub const fn is_valid(&self) -> bool {
            if self.user.display_name.is_empty() || self.user.name.is_empty() {
                return false;
            }

            if let AuthMethod::ApiToken(token) = &self.user.auth_method
                && token.is_empty()
            {
                return false;
            }

            true
        }

        pub fn view(&self) -> Element<'_, Message> {
            let display_name = text_input("Display Name", self.user.display_name.as_str())
                .on_input(Message::DisplayName)
                .id(Self::DISPLAY_NAME_ID);

            let username =
                text_input("Username", self.user.name.as_str()).on_input(Message::Username);

            let password = button("Password");
            let api = button("API Token");

            let buttons = match self.user.auth_method {
                AuthMethod::Password => row![password, api.on_press(Message::Api)],
                AuthMethod::ApiToken(_) => row![password.on_press(Message::Password), api],
            };

            let auth_method = match &self.user.auth_method {
                AuthMethod::Password => None,
                AuthMethod::ApiToken(token) => Some(
                    text_input("API Token", token.as_str())
                        .on_input(Message::Token)
                        .on_submit(Message::Submit),
                ),
            };

            let submit = match &self.user.auth_method {
                AuthMethod::Password => Some(button("Submit").on_press(Message::Submit)),
                AuthMethod::ApiToken(_) => None,
            };

            container(column![display_name, username, buttons, auth_method, submit].align_x(Center))
                .center(400)
                .into()
        }
    }
}
