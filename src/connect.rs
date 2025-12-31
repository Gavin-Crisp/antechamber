use crate::{
    config::{Config, User},
    include_svg,
    modal::modal,
    proxmox::{Auth, Guest, GuestKind, SpiceConfig},
};
use iced::{
    alignment::Horizontal, event::listen_with, widget::{button, center, column, container, scrollable, stack, svg, text}, Center, Element, Fill,
    Shrink,
    Subscription,
    Task,
};

include_svg!(SETTINGS, "lucide/settings.svg");

#[derive(Debug)]
pub struct State {
    auth: Auth,
    guests: Option<Vec<Guest>>,
    user: User,
    show_modal: bool,
}

#[derive(Clone, Debug)]
pub enum Message {
    Auth(Auth),
    GetGuests(Vec<Guest>),
    SpiceConfig(SpiceConfig),
    ConnectHost(u32),
    Logout,
    Settings,
    Modal(settings_modal::Message),
}

#[derive(Debug)]
pub enum Action {
    Logout,
    Run(Task<Message>),
    None,
}

impl State {
    pub fn new(auth: Auth, user: User) -> (Self, Task<Message>) {
        (
            Self {
                auth,
                guests: None,
                user,
                show_modal: false,
            },
            // TODO: Replace with api call
            Task::done(Message::GetGuests(
                (0..6)
                    .map(|i| Guest {
                        name: format!("Guest{i}"),
                        vmid: 100 + i,
                        node: "N1".to_owned(),
                        kind: GuestKind::Qemu,
                    })
                    .collect(),
            )),
        )
    }

    #[allow(clippy::unused_self)]
    pub fn subscription(&self) -> Subscription<Message> {
        // TODO: use this subscription to keepalive auth session
        // This is probably overkill
        listen_with(|_, _, _| None)
    }

    pub fn update(&mut self, message: Message, _config: &mut Config) -> Action {
        match message {
            Message::Auth(auth) => self.auth = auth,
            Message::GetGuests(guests) => self.guests = Some(guests),
            Message::SpiceConfig(_spice_config) => {
                // TODO: start remote viewer with config
            }
            Message::ConnectHost(_vmid) => {
                // TODO: Replace with attempt connection
                return Action::Run(Task::done(Message::SpiceConfig(SpiceConfig {
                    host: String::new(),
                    password: String::new(),
                    proxy: String::new(),
                    tls_port: 0,
                    conn_type: String::new(),
                })));
            }
            Message::Logout => return Action::Logout,
            Message::Settings => self.show_modal = true,
            Message::Modal(message) => {
                if self.show_modal {
                    match settings_modal::update(&mut self.user, message) {
                        settings_modal::Action::Close => self.show_modal = false,
                    }
                }
            }
        }

        Action::None
    }

    pub fn view(&self, _config: &Config) -> Element<'_, Message> {
        let Some(guests) = &self.guests else {
            return center("Getting guests...").into();
        };

        let hosts = container(
            scrollable(
                column(guests.iter().map(view_guest))
                    .align_x(Center)
                    .spacing(4),
            )
            .height(240),
        )
        .padding([50, 20]);

        let logout_button = button("Logout").on_press(Message::Logout);
        let settings_button = button(svg(SETTINGS.clone()))
            .on_press(Message::Settings)
            .width(Shrink);

        let page = column![
            text(&self.user.name).size(25).width(Fill),
            hosts,
            logout_button,
            container(Option::<Element<Message>>::None).height(Fill),
            container(settings_button).width(Fill)
        ]
        .width(Shrink)
        .align_x(Horizontal::Center)
        .padding([25, 50]);

        stack![
            page,
            if self.show_modal {
                Some(modal(
                    settings_modal::view(&self.user).map(Message::Modal),
                    Message::Modal(settings_modal::Message::Close),
                ))
            } else {
                None
            },
        ]
        .width(Fill)
        .into()
    }
}

fn view_guest(guest: &Guest) -> Element<'_, Message> {
    button(column![
        text(guest.name.clone()),
        text(guest.kind.to_string()).size(12.5)
    ])
    .width(170)
    .height(60)
    .padding(10)
    .on_press(Message::ConnectHost(guest.vmid))
    .into()
}

mod settings_modal {
    use crate::config::User;
    use iced::{widget::container, Element};

    #[derive(Clone, Debug)]
    pub enum Message {
        Close,
    }

    pub enum Action {
        Close,
    }

    #[allow(clippy::needless_pass_by_value)]
    pub const fn update(_user: &mut User, message: Message) -> Action {
        match message {
            Message::Close => Action::Close,
        }
    }

    pub fn view(user: &User) -> Element<'_, Message> {
        container(user.name.as_str()).center(400).into()
    }
}
