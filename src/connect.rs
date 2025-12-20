use crate::proxmox::{Auth, Guest, GuestKind, SpiceConfig};
use iced::{
    alignment::Horizontal, event::listen_with, widget::{button, center, column, scrollable, text}, Center,
    Element,
    Subscription,
    Task,
};

#[derive(Debug)]
pub struct State {
    auth: Auth,
    guests: Option<Vec<Guest>>,
}

#[derive(Clone, Debug)]
pub enum Message {
    Auth(Auth),
    GetGuests(Vec<Guest>),
    SpiceConfig(SpiceConfig),
    ConnectHost(u32),
    Logout,
}

#[derive(Debug)]
pub enum Action {
    Logout,
    Run(Task<Message>),
}

impl State {
    pub fn new(auth: Auth) -> (Self, Task<Message>) {
        (
            Self { auth, guests: None },
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

    pub fn update(&mut self, message: Message) -> Action {
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
        }

        Action::Run(Task::none())
    }

    pub fn view(&self) -> Element<'_, Message> {
        let Some(guests) = &self.guests else {
            return center("Getting guests...").into();
        };

        let hosts_column = column(guests.iter().map(view_guest)).align_x(Center);

        let hosts: Element<'_, Message> = if guests.len() > 3 {
            scrollable(hosts_column).height(180).into()
        } else {
            hosts_column.into()
        };

        let logout_button = button("Logout").on_press(Message::Logout);

        center(column![hosts, logout_button].align_x(Horizontal::Center)).into()
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
