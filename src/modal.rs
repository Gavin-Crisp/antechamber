use crate::include_svg;
use iced::{
    color, widget::{button, center, column, container, opaque, svg}, Background, Element, Length, Padding,
    Shrink,
};

include_svg!(CLOSE, "lucide/close.svg");

pub fn modal<'a, Message, Theme>(
    content: impl Into<Element<'a, Message, Theme>>,
    on_close: Message,
) -> Modal<'a, Message, Theme>
where
    Theme: container::Catalog + button::Catalog + svg::Catalog,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
{
    Modal::new(content, on_close)
}

pub struct Modal<'a, Message, Theme>
where
    Theme: container::Catalog + button::Catalog + svg::Catalog,
{
    content: Element<'a, Message, Theme>,
    width: Length,
    height: Length,
    on_close: Message,
    padding: Padding,
    close_padding: Padding,
    overlay: Background,
    box_class: <Theme as container::Catalog>::Class<'a>,
    close_class: <Theme as button::Catalog>::Class<'a>,
    close_svg_class: <Theme as svg::Catalog>::Class<'a>,
}

impl<'a, Message, Theme> Modal<'a, Message, Theme>
where
    Theme: container::Catalog + button::Catalog + svg::Catalog,
{
    pub fn new(content: impl Into<Element<'a, Message, Theme>>, on_close: Message) -> Self
    where
        <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
    {
        let content = content.into();
        let size = content.as_widget().size_hint();

        Self {
            content,
            width: size.width.fluid(),
            height: size.height.fluid(),
            on_close,
            padding: Padding::new(10.0),
            close_padding: Padding::ZERO,
            overlay: color!(0x0, 0.3).into(),
            box_class: <Theme as container::Catalog>::default(),
            close_class: <Theme as button::Catalog>::default(),
            close_svg_class: <Theme as svg::Catalog>::default(),
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

    pub fn close_padding(mut self, padding: impl Into<Padding>) -> Self {
        self.close_padding = padding.into();
        self
    }

    pub fn overlay(mut self, background: impl Into<Background>) -> Self {
        self.overlay = background.into();
        self
    }

    pub fn style(mut self, style: impl Fn(&Theme) -> container::Style + 'a) -> Self
    where
        <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
    {
        self.box_class = (Box::new(style) as container::StyleFn<'a, Theme>).into();
        self
    }

    pub fn box_class(mut self, class: impl Into<<Theme as container::Catalog>::Class<'a>>) -> Self {
        self.box_class = class.into();
        self
    }

    pub fn close_class(mut self, class: impl Into<<Theme as button::Catalog>::Class<'a>>) -> Self {
        self.close_class = class.into();
        self
    }

    pub fn svg_style(mut self, style: impl Fn(&Theme, svg::Status) -> svg::Style + 'a) -> Self
    where
        <Theme as svg::Catalog>::Class<'a>: From<svg::StyleFn<'a, Theme>>,
    {
        self.close_svg_class = (Box::new(style) as svg::StyleFn<'a, Theme>).into();
        self
    }
}

impl<'a, Message: Clone + 'a, Theme> From<Modal<'a, Message, Theme>> for Element<'a, Message, Theme>
where
    Theme: container::Catalog + button::Catalog + svg::Catalog + 'a,
    <Theme as container::Catalog>::Class<'a>: From<container::StyleFn<'a, Theme>>,
    <Theme as button::Catalog>::Class<'a>: From<button::StyleFn<'a, Theme>>,
    <Theme as svg::Catalog>::Class<'a>: From<svg::StyleFn<'a, Theme>>,
{
    fn from(value: Modal<'a, Message, Theme>) -> Self {
        let close = button(svg(CLOSE.clone()).class(value.close_svg_class))
            .width(Shrink)
            .padding(value.close_padding)
            .on_press(value.on_close)
            .class(value.close_class);

        opaque(
            center(
                container(column![close, value.content])
                    .width(value.width)
                    .height(value.height)
                    .padding(value.padding)
                    .class(value.box_class),
            )
            .style(move |_theme| container::Style::default().background(value.overlay)),
        )
    }
}
