use iced::widget::center;
use iced::{
    widget::{button, row, svg, text_input, Svg}, Element, Fill,
    Shrink,
};

#[derive(Debug)]
pub struct State {
    value: String,
    secure: bool,
    placeholder: String,
}

#[derive(Clone, Debug)]
pub enum Message {
    UpdateValue(String),
    ToggleSecure,
    Submit,
    Clear,
}

impl State {
    pub fn new(placeholder: impl Into<String>) -> Self {
        Self {
            value: String::new(),
            secure: true,
            placeholder: placeholder.into(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::UpdateValue(value) => self.value = value,
            Message::ToggleSecure => self.secure = !self.secure,
            Message::Submit => {}
            Message::Clear => self.value.clear(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let text_box = text_input(&self.placeholder, &self.value)
            .on_input(Message::UpdateValue)
            .on_submit(Message::Submit)
            .secure(self.secure);

        let open_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/eye.svg");
        let closed_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/eye-off.svg");
        let show_icon: Svg = svg(if self.secure { open_path } else { closed_path });
        let show_button = button(center(show_icon))
            .width(25)
            .height(Fill)
            .padding(2)
            .on_press(Message::ToggleSecure);

        row![text_box, show_button]
            .height(Shrink)
            .into()
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new("Password")
    }
}
