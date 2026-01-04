use crate::{
    config::{AuthMethod, Config},
    include_svg,
    modal::modal,
    proxmox::Auth,
};
use iced::{
    alignment::Horizontal, mouse::Interaction, widget::{
        button, center, column, container, mouse_area, pick_list, row, stack, svg, text_input, Svg,
    }, Element,
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
    password: String,
    secure_password: bool,
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
    pub const fn new(config: &Config) -> Self {
        Self {
            modal: None,
            cluster: config.default_cluster,
            user: config.default_user,
            password: String::new(),
            secure_password: true,
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
                    self.select_user(new);
                }
                Action::None
            }
            Message::ShowModal => {
                self.modal = Some(user_modal::State::default());
                Action::None
            }
            Message::Modal(message) => {
                if let Some(state) = &mut self.modal {
                    match state.update(message) {
                        user_modal::Action::Add(user) => {
                            self.modal = None;
                            config.users.push(user);
                            self.select_user(config.users.len() - 1);

                            return Action::SaveConfig;
                        }
                        user_modal::Action::Close => self.modal = None,
                        user_modal::Action::None => {}
                    }
                }
                Action::None
            }
            Message::Password(password) => {
                self.password = password;
                Action::None
            }
            Message::ShowPassword => {
                self.secure_password = false;
                Action::None
            }
            Message::HidePassword => {
                self.secure_password = true;
                Action::None
            }
            Message::SubmitPassword => {
                if self.user.is_some() && !self.password.is_empty() {
                    // TODO: replace with password login
                    Action::Run(Task::done(Message::Login(Auth {
                        csrf: String::new(),
                        ticket: String::new(),
                    })))
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

    fn select_user(&mut self, user: usize) {
        self.user = Some(user);
        self.password.clear();
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

        let auth: Option<Element<Message>> =
            self.user.map(|user| match config.users[user].auth_method {
                AuthMethod::Password => {
                    let password_input = text_input("Password", &self.password)
                        .on_input(Message::Password)
                        .on_submit(Message::SubmitPassword)
                        .secure(self.secure_password);

                    let eye_svg: Svg = svg(if self.secure_password {
                        OPEN_EYE.clone()
                    } else {
                        CLOSED_EYE.clone()
                    });
                    let show_button =
                        mouse_area(container(eye_svg).center_x(35).center_y(Fill).padding(5))
                            .on_press(Message::ShowPassword)
                            .on_release(Message::HidePassword)
                            .interaction(Interaction::Pointer);

                    row![password_input, show_button].height(Shrink).into()
                }
                AuthMethod::ApiToken(_) => button("Login").on_press(Message::SubmitApi).into(),
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
        widget::{button, column, container, row, text_input}, Center,
        Element,
    };
    use std::mem;

    #[derive(Clone, Debug, Default)]
    pub struct State {
        user: User,
    }

    #[derive(Clone, Debug)]
    pub enum Message {
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
        pub fn update(&mut self, message: Message) -> Action {
            match message {
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
                    if self.is_valid() {
                        Action::Add(mem::take(&mut self.user))
                    } else {
                        Action::None
                    }
                }
            }
        }

        pub const fn is_valid(&self) -> bool {
            if self.user.name.is_empty() {
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

            container(column![username, buttons, auth_method, submit].align_x(Center))
                .center(400)
                .into()
        }
    }
}
