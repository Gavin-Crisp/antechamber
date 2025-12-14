use iced::{
    alignment::Horizontal, widget::{
        button, center, column, keyed_column, scrollable,
        scrollable::{Direction, Scrollbar},
    },
    Element,
    Task,
};

#[derive(Debug)]
pub struct State {
    guests: Vec<Guest>,
}

#[derive(Debug)]
pub struct Guest {
    pub name: String,
    pub vmid: u32,
    pub node: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    ConnectHost(usize),
    Logout,
}

#[derive(Debug)]
pub enum Action {
    Logout,
    Run(Task<Message>),
}

impl State {
    pub const fn new(hosts: Vec<Guest>) -> Self {
        Self { guests: hosts }
    }

    #[allow(clippy::needless_pass_by_ref_mut)]
    #[allow(clippy::needless_pass_by_value)]
    #[allow(clippy::unused_self)]
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::ConnectHost(_index) => {
                // TODO: Replace with attempt connection
                Action::Run(Task::none())
            }
            Message::Logout => Action::Logout,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let hosts = scrollable(keyed_column(
            self.guests
                .iter()
                .enumerate()
                .map(|(id, host)| (id, view_guest(id, host))),
        ))
        .height(150)
        .direction(Direction::Vertical(Scrollbar::hidden()));

        let logout_button = button("Logout").on_press(Message::Logout);

        center(column![hosts, logout_button].align_x(Horizontal::Center)).into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new(
            (0..20)
                .map(|i| Guest {
                    name: format!("Guest{i}"),
                    vmid: 100 + i,
                    node: "N1".to_owned(),
                })
                .collect(),
        )
    }
}

fn view_guest(key: usize, guest: &Guest) -> Element<'_, Message> {
    button(guest.name.as_str())
        .on_press(Message::ConnectHost(key))
        .padding(10)
        .into()
}
