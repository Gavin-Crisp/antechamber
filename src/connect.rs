use iced::{
    alignment::Horizontal, widget::{button, center, column, keyed_column},
    Element,
    Task,
};

#[derive(Debug)]
pub struct State {
    hosts: Vec<Host>,
}

type Host = String;

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
    pub const fn new(hosts: Vec<Host>) -> Self {
        Self { hosts }
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
        let hosts = keyed_column(
            self.hosts
                .iter()
                .enumerate()
                .map(|(id, host)| (id, view_host(id, host))),
        );

        let logout_button = button("Logout").on_press(Message::Logout);

        center(column![hosts, logout_button].align_x(Horizontal::Center)).into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new(vec!["Test1".to_owned(), "Test2".to_owned()])
    }
}

fn view_host(key: usize, name: &Host) -> Element<'_, Message> {
    button(name.as_str())
        .on_press(Message::ConnectHost(key))
        .padding(10)
        .into()
}
