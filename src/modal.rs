use crate::include_svg;
use iced::{
    widget::{button, center, column, container, opaque, svg}, Background, Element, Length, Padding,
    Shrink,
};

include_svg!(CLOSE, "lucide/close.svg");

pub fn modal<'a, Message, Theme: container::Catalog>(
    content: impl Into<Element<'a, Message, Theme>>,
    on_close: Message,
) -> Modal<'a, Message, Theme> {
    Modal::new(content, on_close)
}

pub struct Modal<'a, Message, Theme: container::Catalog> {
    content: Element<'a, Message, Theme>,
    width: Length,
    height: Length,
    on_close: Message,
    padding: Padding,
    background: Option<Background>,
    class: Theme::Class<'a>,
}

impl<'a, Message, Theme: container::Catalog> Modal<'a, Message, Theme> {
    pub fn new(content: impl Into<Element<'a, Message, Theme>>, on_close: Message) -> Self {
        let content = content.into();
        let size = content.as_widget().size_hint();

        Self {
            content,
            width: size.width.fluid(),
            height: size.height.fluid(),
            on_close,
            padding: Padding::default(),
            background: None,
            class: Theme::default(),
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn background(mut self, background: impl Into<Background>) -> Self {
        self.background = Some(background.into());
        self
    }

    pub fn style(mut self, style: impl Fn(&Theme) -> container::Style + 'a) -> Self
    where
        Theme::Class<'a>: From<container::StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as container::StyleFn<'a, Theme>).into();
        self
    }

    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<'a, Message: Clone + 'a, Theme: container::Catalog + button::Catalog + svg::Catalog + 'a>
    From<Modal<'a, Message, Theme>> for Element<'a, Message, Theme>
{
    fn from(value: Modal<'a, Message, Theme>) -> Self {
        let close = button(svg(CLOSE.clone()))
            .on_press(value.on_close)
            .width(Shrink);

        opaque(center(
            container(column![close, value.content])
                .width(value.width)
                .height(value.height)
                .class(value.class),
        ))
    }
}
