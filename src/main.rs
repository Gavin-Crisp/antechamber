mod connect;
mod login;

use iced::{
    keyboard::{self, key::Named, Key}, widget::operation, Element,
    Subscription,
    Task,
};

fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .subscription(State::subscription)
        .run()
}

#[derive(Debug)]
enum State {
    Login(login::State),
    Connect(connect::State),
}

#[derive(Debug)]
enum Message {
    Login(login::Message),
    Connect(connect::Message),
    Event(keyboard::Event),
}

impl State {
    #[allow(clippy::unused_self)]
    pub fn subscription(&self) -> Subscription<Message> {
        keyboard::listen().map(Message::Event)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Login(message) => {
                if let Self::Login(state) = self {
                    match state.update(message) {
                        login::Action::Login => {
                            *self = Self::Connect(connect::State::default());
                            // TODO: Replace with get guests request
                            Task::none()
                        }
                        login::Action::Run(task) => task.map(Message::Login),
                    }
                } else {
                    Task::none()
                }
            }
            Message::Connect(message) => {
                if let Self::Connect(state) = self {
                    match state.update(message) {
                        connect::Action::Logout => {
                            *self = Self::Login(login::State::default());
                            Task::none()
                        }
                        connect::Action::Run(task) => task.map(Message::Connect),
                    }
                } else {
                    Task::none()
                }
            }
            Message::Event(event) => {
                if let keyboard::Event::KeyPressed {
                    key: Key::Named(Named::Tab),
                    modifiers,
                    ..
                } = event
                {
                    if modifiers.shift() {
                        operation::focus_previous()
                    } else {
                        operation::focus_next()
                    }
                } else {
                    Task::none()
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self {
            Self::Login(state) => state.view().map(Message::Login),
            Self::Connect(state) => state.view().map(Message::Connect),
        }
        .explain(iced::color!(0xcc_cc_cc))
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Login(login::State::default())
    }
}
