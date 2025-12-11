use iced::Element;

fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .centered()
        .run()
}

#[derive(Default)]
struct State {}

enum Message {}

impl State {
    #[allow(clippy::unused_self)]
    #[allow(clippy::needless_pass_by_value)]
    pub const fn update(&mut self, message: Message) {
        _ = message;
    }

    #[allow(clippy::unused_self)]
    pub fn view(&self) -> Element<'_, Message> {
        "".into()
    }
}
