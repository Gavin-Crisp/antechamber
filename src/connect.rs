use iced::{
    alignment::Horizontal, event::listen_with, widget::{button, center, column, scrollable, text}, Center,
    Element,
    Subscription,
    Task,
};
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct State {
    guests: Option<Vec<Guest>>,
}

#[derive(Clone, Debug)]
pub struct Guest {
    pub name: String,
    pub vmid: u32,
    pub node: String,
    pub engine: Engine,
}

#[derive(Clone, Debug)]
pub enum Engine {
    Qemu,
    Lxc,
}

impl Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Qemu => write!(f, "Qemu"),
            Self::Lxc => write!(f, "LXC"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    GetGuests(Vec<Guest>),
    ConnectHost(usize),
    Logout,
}

#[derive(Debug)]
pub enum Action {
    Logout,
    Run(Task<Message>),
}

impl State {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                // TODO: Replace with None once api calls added
                guests: Some(
                    (0..6)
                        .map(|i| Guest {
                            name: format!("Guest{i}"),
                            vmid: 100 + i,
                            node: "N1".to_owned(),
                            engine: Engine::Qemu,
                        })
                        .collect(),
                ),
            },
            // TODO: Replace with fetch vms
            Task::none(),
        )
    }

    #[allow(clippy::unused_self)]
    pub fn subscription(&self) -> Subscription<Message> {
        // TODO: use this subscription to keepalive auth session
        // This is probably overkill
        listen_with(|_, _, _| None)
    }

    #[allow(clippy::needless_pass_by_ref_mut)]
    #[allow(clippy::needless_pass_by_value)]
    #[allow(clippy::unused_self)]
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::GetGuests(guests) => {
                self.guests = Some(guests);
                Action::Run(Task::none())
            }
            Message::ConnectHost(_index) => {
                // TODO: Replace with attempt connection
                Action::Run(Task::none())
            }
            Message::Logout => Action::Logout,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let Some(guests) = &self.guests else {
            return center("Getting guests...").into();
        };

        let hosts_column = column(
            guests
                .iter()
                .enumerate()
                .map(|(id, host)| view_guest(id, host)),
        )
        .align_x(Center);

        let hosts: Element<'_, Message> = if guests.len() > 3 {
            scrollable(hosts_column).height(180).into()
        } else {
            hosts_column.into()
        };

        let logout_button = button("Logout").on_press(Message::Logout);

        center(column![hosts, logout_button].align_x(Horizontal::Center)).into()
    }
}

fn view_guest(key: usize, guest: &Guest) -> Element<'_, Message> {
    button(column![
        text(guest.name.clone()),
        text(guest.engine.to_string()).size(12.5)
    ])
    .width(170)
    .height(60)
    .padding(10)
    .on_press(Message::ConnectHost(key))
    .into()
}
