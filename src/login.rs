use crate::{include_svg, proxmox::Auth};
use iced::{
    alignment::Horizontal, mouse::Interaction, widget::{center, column, container, mouse_area, row, svg, text_input, Svg}, Element,
    Fill,
    Shrink,
    Task,
};

include_svg!(OPEN_EYE, "eye.svg");
include_svg!(CLOSED_EYE, "eye-off.svg");

#[derive(Debug)]
pub struct State {
    username: String,
    password: String,
    secure_password: bool,
}

#[derive(Clone, Debug)]
pub enum Message {
    Username(String),
    Password(String),
    ShowPassword,
    HidePassword,
    Submit,
    Auth(Auth),
}

#[derive(Debug)]
pub enum Action {
    Login(Auth),
    Run(Task<Message>),
}

impl State {
    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::Username(username) => self.username = username,
            Message::Password(password) => self.password = password,
            Message::ShowPassword => self.secure_password = false,
            Message::HidePassword => self.secure_password = true,
            Message::Submit => {
                // TODO: replace with attempt login
                return Action::Run(Task::done(Message::Auth(Auth {
                    ticket: String::new(),
                    csrf: String::new(),
                })));
            }
            Message::Auth(auth) => {
                return Action::Login(auth);
            }
        }

        Action::Run(Task::none())
    }

    pub fn view(&self) -> Element<'_, Message> {
        let username_input = text_input("Username", &self.username).on_input(Message::Username);
        let password_input = text_input("Password", &self.password)
            .on_input(Message::Password)
            .on_submit(Message::Submit)
            .secure(self.secure_password);

        let eye_svg: Svg = svg(if self.secure_password {
            OPEN_EYE.clone()
        } else {
            CLOSED_EYE.clone()
        });
        let show_button = mouse_area(container(eye_svg).center_x(35).center_y(Fill).padding(5))
            .on_press(Message::ShowPassword)
            .on_release(Message::HidePassword)
            .interaction(Interaction::Pointer);

        let input_box = column![
            username_input,
            row![password_input, show_button].height(Shrink)
        ]
        .padding(10)
        .spacing(10)
        .align_x(Horizontal::Center)
        .width(300)
        .padding(10);

        center(input_box).into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            secure_password: true,
        }
    }
}
