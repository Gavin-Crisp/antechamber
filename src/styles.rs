use iced::{border::Radius, widget::container, Border, Color, Shadow, Theme};

pub fn ui_box(theme: &Theme) -> container::Style {
    let extended = theme.extended_palette();

    container::Style {
        text_color: Some(extended.background.neutral.text),
        background: Some(extended.background.neutral.color.into()),
        border: Border {
            color: Color::default(),
            width: 0.0,
            radius: Radius::new(30),
        },
        shadow: Shadow::default(),
        snap: false,
    }
}
