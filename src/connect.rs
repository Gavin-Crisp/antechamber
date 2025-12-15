use iced::{
    alignment::Horizontal, widget::{
        button, center, column, keyed_column, scrollable,
        scrollable::{Direction, Scrollbar},
    }, Alignment,
    Element,
    Task,
};

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
                    (0..20)
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

        let hosts = scrollable(
            keyed_column(
                guests
                    .iter()
                    .enumerate()
                    .map(|(id, host)| (id, view_guest(id, host))),
            )
            .align_items(Alignment::Center),
        )
        .height(150)
        .direction(Direction::Vertical(Scrollbar::hidden()));

        let logout_button = button("Logout").on_press(Message::Logout);

        center(column![hosts, logout_button].align_x(Horizontal::Center)).into()
    }
}

fn view_guest(key: usize, guest: &Guest) -> Element<'_, Message> {
    button(guest.name.as_str())
        .on_press(Message::ConnectHost(key))
        .padding(10)
        .into()
}
