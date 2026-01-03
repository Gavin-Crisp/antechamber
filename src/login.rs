use crate::{
    config::{AuthMethod, Cluster, Config, User},
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
    user: Option<User>,
    password: String,
    secure_password: bool,
}

#[derive(Clone, Debug)]
pub enum Message {
    SelectCluster(usize),
    SelectUser(User),
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
    Login(Auth, User),
    Run(Task<Message>),
    SaveConfig,
    None,
}

impl State {
    pub fn new(config: &Config) -> Self {
        let cluster = config.default_cluster;
        let user = cluster.and_then(|idx| config.clusters[idx].users.first().cloned());

        Self {
            modal: None,
            cluster,
            user,
            password: String::new(),
            secure_password: true,
        }
    }

    pub fn update(&mut self, message: Message, clusters: &mut [Cluster]) -> Action {
        match message {
            Message::SelectCluster(cluster) => {
                if self.cluster.is_none_or(|current| current != cluster) {
                    self.cluster = Some(cluster);

                    if let Some(user) = clusters[cluster]
                        .default_user
                        .and_then(|idx| clusters[idx].users.first().cloned())
                    {
                        self.select_user(user);
                    } else {
                        self.user = None;
                    }
                }
            }
            Message::SelectUser(user) => {
                if self
                    .user
                    .as_ref()
                    .is_none_or(|current_user| current_user != &user)
                {
                    self.select_user(user);
                }
            }
            Message::ShowModal => self.modal = Some(user_modal::State::default()),
            Message::Modal(message) => {
                if let Some(state) = &mut self.modal {
                    match state.update(message) {
                        user_modal::Action::Add(user) => {
                            self.modal = None;

                            if let Some(index) = self.cluster {
                                clusters[index].users.push(user.clone());
                                self.select_user(user);

                                return Action::SaveConfig;
                            }
                        }
                        user_modal::Action::Close => self.modal = None,
                        user_modal::Action::None => {}
                    }
                }
            }
            Message::Password(password) => self.password = password,
            Message::ShowPassword => self.secure_password = false,
            Message::HidePassword => self.secure_password = true,
            Message::SubmitPassword => {
                // TODO: replace with password login
                return Action::Run(Task::done(Message::Login(Auth {
                    csrf: String::new(),
                    ticket: String::new(),
                })));
            }
            Message::SubmitApi => {
                // TODO: replace with api login
                return Action::Run(Task::done(Message::Login(Auth {
                    ticket: String::new(),
                    csrf: String::new(),
                })));
            }
            Message::Login(auth) => {
                if let Some(user) = self.user.take() {
                    return Action::Login(auth, user);
                }
            }
        }

        Action::None
    }

    fn select_user(&mut self, user: User) {
        self.user = Some(user);
        self.password.clear();
    }

    pub fn view<'a>(&'a self, clusters: &'a [Cluster]) -> Element<'a, Message> {
        let cluster_select = pick_list(
            clusters,
            self.cluster.map(|idx| clusters[idx].clone()),
            |cluster| {
                Message::SelectCluster(
                    clusters
                        .iter()
                        .position(|c| c.name == cluster.name)
                        .expect("cluster is in clusters"),
                )
            },
        )
        .placeholder("Select cluster");

        let user = self.cluster.map(|cluster| {
            let user_select = pick_list(
                clusters[cluster].users.as_slice(),
                self.user.as_ref(),
                Message::SelectUser,
            )
            .placeholder("Select user");

            let add_user = button(svg(ADD_USER.clone()))
                .width(Shrink)
                .on_press(Message::ShowModal);

            row![user_select, add_user]
        });

        let auth: Option<Element<Message>> =
            self.user.as_ref().map(|user| match user.auth_method {
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
                Message::Username(name) => self.user.name = name,
                Message::Password => self.user.auth_method = AuthMethod::Password,
                Message::Api => self.user.auth_method = AuthMethod::ApiToken(String::new()),
                Message::Token(token) => {
                    if let AuthMethod::ApiToken(curr_token) = &mut self.user.auth_method {
                        *curr_token = token;
                    }
                }
                Message::Close => return Action::Close,
                Message::Submit => {
                    if self.is_valid() {
                        return Action::Add(mem::take(&mut self.user));
                    }
                }
            }

            Action::None
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
