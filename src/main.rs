mod login;

use iced::{Element, Task};

fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view).run()
}

#[derive(Debug)]
enum State {
    Login(login::State),
}

#[derive(Debug)]
enum Message {
    Login(login::Message),
}

impl State {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Login(message) => {
                if let Self::Login(state) = self {
                    state.update(message).map(Message::Login)
                } else {
                    Task::none()
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self {
            Self::Login(state) => state.view().map(Message::Login),
        }
        // .explain(color!(0xcc_cc_cc))
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Login(login::State::default())
    }
}
