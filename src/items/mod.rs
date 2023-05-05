mod navigation_item;
pub mod select;

pub use navigation_item::NavigationItem;
pub use select::Select;

use crate::{
    margin::{Margin, MarginExt},
    MenuEvent, MenuItem, MenuStyle,
};
use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::Rgb888,
    prelude::{PixelColor, Point, Size},
    primitives::{Rectangle, StyledDrawable},
    text::{renderer::TextRenderer, Baseline},
    Drawable,
};
use embedded_layout::prelude::*;
use embedded_text::{alignment::HorizontalAlignment, style::TextBoxStyleBuilder, TextBox};

pub struct MenuLine<I> {
    pub(crate) bounds: Margin<Rectangle>,
    pub(crate) item: I,
}

impl<I> MenuLine<I> {
    pub fn new<C>(item: I, style: MenuStyle<C>) -> MenuLine<I>
    where
        C: PixelColor,
    {
        let style = style.text_style();

        MenuLine {
            item,
            bounds: Rectangle::new(
                Point::zero(),
                Size::new(1, style.font.character_size.height),
            )
            .with_margin(0, 0, -1, 1),
        }
    }
}

impl<I> View for MenuLine<I> {
    fn translate_impl(&mut self, by: Point) {
        self.bounds.translate_mut(by);
    }

    fn bounds(&self) -> Rectangle {
        self.bounds.bounds()
    }
}

impl<I> MenuItem for MenuLine<I>
where
    I: MenuItem,
{
    type Data = I::Data;

    fn interact(&mut self) -> MenuEvent<Self::Data> {
        self.item.interact()
    }

    fn title(&self) -> &str {
        self.item.title()
    }

    fn details(&self) -> &str {
        self.item.details()
    }

    fn value(&self) -> &str {
        self.item.value()
    }
}

impl<C, I> StyledDrawable<MenuStyle<C>> for MenuLine<I>
where
    C: PixelColor + From<Rgb888>,
    I: MenuItem,
{
    type Color = C;
    type Output = ();

    fn draw_styled<D>(
        &self,
        style: &MenuStyle<C>,
        display: &mut D,
    ) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let text_bounds = self.bounds.bounds();
        let display_area = display.bounding_box();

        if text_bounds.intersection(&display_area).is_zero_sized() {
            return Ok(());
        }

        let text_style = style.text_style();
        let value_text = self.item.value();

        let mut inner_bounds = self.bounds.inner.bounds();
        inner_bounds.size.width = display_area.size.width - self.bounds.left as u32;
        TextBox::with_textbox_style(
            value_text,
            inner_bounds,
            text_style,
            TextBoxStyleBuilder::new()
                .alignment(HorizontalAlignment::Right)
                .build(),
        )
        .draw(display)?;

        let value_width = text_style
            .measure_string(value_text, Point::zero(), Baseline::Top)
            .bounding_box
            .size
            .width;

        inner_bounds.size.width -= value_width;
        TextBox::new(self.item.title(), inner_bounds, text_style).draw(display)?;

        Ok(())
    }
}
