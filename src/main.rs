mod connect;
mod login;

use iced::{
    event::{self, listen_with, Status}, keyboard::{self, key::Named, Key}, widget::operation,
    Element,
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
    FocusNext,
    FocusPrev,
}

impl State {
    #[allow(clippy::unused_self)]
    pub fn subscription(&self) -> Subscription<Message> {
        listen_with(|event, status, _id| {
            if status == Status::Captured {
                return None;
            }

            let event::Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::Tab),
                modifiers,
                ..
            }) = event
            else {
                return None;
            };

            if modifiers.shift() {
                Some(Message::FocusPrev)
            } else {
                Some(Message::FocusNext)
            }
        })
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Login(message) => {
                if let Self::Login(state) = self {
                    match state.update(message) {
                        login::Action::Login => {
                            let (state, task) = connect::State::new();
                            *self = Self::Connect(state);
                            task.map(Message::Connect)
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
            Message::FocusNext => operation::focus_next(),
            Message::FocusPrev => operation::focus_previous(),
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
