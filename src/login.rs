use crate::{
    config::{AuthMethod, Cluster, Config, User},
    include_svg,
    proxmox::Auth,
};
use iced::{
    alignment::Horizontal, mouse::Interaction, widget::{button, center, column, container, mouse_area, pick_list, row, svg, text_input, Svg}, Element,
    Fill,
    Shrink,
    Task,
};

include_svg!(OPEN_EYE, "lucide/eye.svg");
include_svg!(CLOSED_EYE, "lucide/eye-off.svg");
include_svg!(ADD_USER, "lucide/user-plus.svg");

#[derive(Debug)]
pub struct State {
    cluster: Option<Cluster>,
    user: Option<User>,
    password: String,
    secure_password: bool,
}

#[derive(Clone, Debug)]
pub enum Message {
    SelectCluster(Cluster),
    SelectUser(User),
    AddUser(User),
    Password(String),
    ShowPassword,
    HidePassword,
    SubmitPassword,
    SubmitApi,
    Login(Auth),
}

#[derive(Debug)]
pub enum Action {
    Login(Auth),
    Run(Task<Message>),
}

impl State {
    pub fn new(config: &Config) -> Self {
        let cluster = config
            .default_cluster
            .and_then(|index| config.clusters.get(index).cloned());
        let user = cluster
            .as_ref()
            .and_then(|cluster| cluster.users.first().cloned());

        Self {
            cluster,
            user,
            password: String::new(),
            secure_password: false,
        }
    }

    pub fn update(&mut self, message: Message, config: &mut Config) -> Action {
        match message {
            Message::SelectCluster(cluster) => {
                if self
                    .cluster
                    .as_ref()
                    .is_none_or(|current_cluster| current_cluster != &cluster)
                {
                    self.cluster = Some(cluster);
                    self.user = None;
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
            Message::ToggleModal => match self.modal {
                None => self.modal = Some(user_modal::State::default()),
                Some(_) => self.modal = None,
            },
            Message::AddUser(user) => {
                if let Some(current_cluster) = &mut self.cluster
                    && let Some(index) = config
                        .clusters
                        .iter_mut()
                        .position(|cluster| cluster.name == current_cluster.name)
                {
                    current_cluster.users.push(user.clone());
                    config.clusters[index].users.push(user.clone());
                    self.select_user(user);
                    
                    // TODO: save config changes
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
                return Action::Login(auth);
            }
        }

        Action::Run(Task::none())
    }

    fn select_user(&mut self, user: User) {
        self.user = Some(user);
        self.password.clear();
    }

    pub fn view<'a>(&'a self, config: &'a Config) -> Element<'a, Message> {
        let cluster_select = pick_list(
            config.clusters.as_slice(),
            self.cluster.as_ref(),
            Message::SelectCluster,
        )
        .placeholder("Select cluster");

        let user = self.cluster.as_ref().map(|cluster| {
            let user_select = pick_list(
                cluster.users.as_slice(),
                self.user.as_ref(),
                Message::SelectUser,
            )
            .placeholder("Select user");

            let add_user = button(svg(ADD_USER.clone())).width(Shrink);

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

        center(input_box).into()
    }
}
