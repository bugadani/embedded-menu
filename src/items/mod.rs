mod navigation_item;
pub mod select;

pub use navigation_item::NavigationItem;
pub use select::Select;

use crate::{margin::Margin, MenuItem};
use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::MonoTextStyle,
    pixelcolor::Rgb888,
    prelude::{PixelColor, Point},
    primitives::Rectangle,
    Drawable,
};
use embedded_layout::prelude::*;
use embedded_text::{alignment::HorizontalAlignment, style::TextBoxStyleBuilder, TextBox};

pub struct MenuLine<'a, C, I> {
    pub(crate) bounds: Margin<Rectangle>,
    pub(crate) text_style: MonoTextStyle<'a, C>,
    pub(crate) item: I,
}

impl<'a, C: PixelColor, I> View for MenuLine<'a, C, I> {
    fn translate_impl(&mut self, by: Point) {
        self.bounds.translate_mut(by);
    }

    fn bounds(&self) -> Rectangle {
        self.bounds.bounds()
    }
}

impl<C, I> Drawable for MenuLine<'_, C, I>
where
    C: PixelColor + From<Rgb888>,
    I: MenuItem,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let text_bounds = self.bounds.bounds();
        let display_area = display.bounding_box();

        if text_bounds.intersection(&display_area).is_zero_sized() {
            return Ok(());
        }

        let mut inner_bounds = self.bounds.inner.bounds();

        inner_bounds.size.width = display_area.size.width - self.bounds.left as u32;

        TextBox::new(self.item.title(), inner_bounds, self.text_style).draw(display)?;

        TextBox::with_textbox_style(
            self.item.value(),
            inner_bounds,
            self.text_style,
            TextBoxStyleBuilder::new()
                .alignment(HorizontalAlignment::Right)
                .build(),
        )
        .draw(display)?;

        Ok(())
    }
}

impl<C, I> MenuItem for MenuLine<'_, C, I>
where
    I: MenuItem,
{
    type Data = I::Data;

    fn interact(&mut self) -> crate::MenuEvent<Self::Data> {
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
