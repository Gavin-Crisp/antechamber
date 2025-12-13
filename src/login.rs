mod password_input;

use iced::{
    alignment::Horizontal, widget::{center, column, text_input},
    Element,
    Task,
};

#[derive(Debug, Default)]
pub struct State {
    username: String,
    password: password_input::State,
}

#[derive(Clone, Debug)]
pub enum Message {
    UpdateUsername(String),
    Password(password_input::Message),
}

impl State {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UpdateUsername(username) => self.username = username,
            Message::Password(password_input::Message::Submit) => {
                // TODO: replace with attempt login.
                self.password.update(password_input::Message::Clear);
            }
            Message::Password(message) => self.password.update(message),
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let username_box = text_input("Username", &self.username).on_input(Message::UpdateUsername);
        let password_box = self.password.view().map(Message::Password);

        let input_box = column![username_box, password_box]
            .spacing(10)
            .align_x(Horizontal::Center)
            .width(300)
            .padding(10);

        center(input_box).into()
    }
}
