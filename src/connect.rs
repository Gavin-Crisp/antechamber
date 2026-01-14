use crate::{
    config::{Config, User},
    include_svg,
    proxmox::{Auth, Guest, GuestKind, SpiceConfig, Ticket},
    styles::ui_box,
};
use iced::{
    alignment::Horizontal, time::{every, minutes}, widget::{button, center, column, container, scrollable, stack, svg, text}, Center, Element, Fill, Shrink,
    Subscription,
    Task,
    Theme,
};

include_svg!(SETTINGS, "lucide/settings.svg");

#[derive(Debug)]
pub struct State {
    auth: Auth,
    guests: Option<Vec<Guest>>,
    cluster: usize,
    user: usize,
    modal: Option<User>,
}

#[derive(Clone, Debug)]
pub enum Message {
    Ticket(Ticket),
    GetGuests(Vec<Guest>),
    SpiceConfig(SpiceConfig),
    ConnectHost(u32),
    Logout,
    Settings,
    Modal(settings_modal::Message),
}

#[derive(Debug)]
pub enum Action {
    Logout(usize),
    Run(Task<Message>),
    None,
}

impl State {
    pub fn new(auth: Auth, cluster: usize, user: usize) -> (Self, Task<Message>) {
        (
            Self {
                auth,
                guests: None,
                cluster,
                user,
                modal: None,
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

    pub fn subscription(&self) -> Subscription<Message> {
        match &self.auth {
            Auth::ApiToken(_) => Subscription::none(),
            Auth::Ticket(_) => {
                // Tickets have a lifetime of 2 hours, so they have to be renewed before then
                every(minutes(110)).map(|_| {
                    // TODO: add api request
                    Message::Ticket(Ticket {
                        ticket: String::new(),
                        csrf: String::new(),
                    })
                })
            }
        }
    }

    pub fn update(&mut self, message: Message, config: &Config) -> Action {
        match message {
            Message::Ticket(ticket) => {
                self.auth = Auth::Ticket(ticket);

                Action::None
            }
            Message::GetGuests(guests) => {
                self.guests = Some(guests);

                Action::None
            }
            Message::SpiceConfig(_spice_config) => {
                // TODO: start remote viewer with config
                Action::None
            }
            Message::ConnectHost(_vmid) => {
                // TODO: Replace with attempt connection
                Action::Run(Task::done(Message::SpiceConfig(SpiceConfig {
                    host: String::new(),
                    password: String::new(),
                    proxy: String::new(),
                    tls_port: 0,
                    conn_type: String::new(),
                })))
            }
            Message::Logout => Action::Logout(self.user),
            Message::Settings => {
                self.modal = Some(config.users[self.user].clone());

                Action::None
            }
            Message::Modal(message) => {
                if let Some(user) = &mut self.modal {
                    match settings_modal::update(user, message) {
                        settings_modal::Action::Close => self.modal = None,
                    }
                }

                Action::None
            }
        }
    }

    pub fn view<'a>(&'a self, config: &'a Config) -> Element<'a, Message> {
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
        let settings_button = button(svg(SETTINGS.clone()).style(|theme: &Theme, _| svg::Style {
            color: Some(theme.extended_palette().primary.base.text),
        }))
        .on_press(Message::Settings)
        .width(Shrink);

        let menu = container(
            column![
                text(config.users[self.user].to_string())
                    .size(25)
                    .width(Fill),
                hosts,
                logout_button,
                container(Option::<Element<Message>>::None).height(Fill),
                container(settings_button).width(Fill)
            ]
            .align_x(Horizontal::Center),
        )
        .width(Shrink)
        .padding([25, 50])
        .style(ui_box);

        stack![
            container(menu).padding(20),
            self.modal
                .as_ref()
                .map(|user| settings_modal::view(user).map(Message::Modal))
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
    use crate::{config::User, modal::modal, styles::ui_box};
    use iced::widget::svg;
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
        modal(
            container(user.display_name.as_str()).center(400),
            Message::Close,
        )
        .style(ui_box)
        .svg_style(|theme, _| svg::Style {
            color: Some(theme.extended_palette().primary.base.text),
        })
        .into()
    }
}
