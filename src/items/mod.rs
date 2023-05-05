mod navigation_item;
pub mod select;

pub use navigation_item::NavigationItem;
pub use select::Select;

use crate::{margin::Margin, MenuItem};
use embedded_graphics::{
    draw_target::DrawTarget, mono_font::MonoTextStyle, pixelcolor::Rgb888, prelude::PixelColor,
    primitives::Rectangle, Drawable,
};
use embedded_layout::prelude::*;
use embedded_text::{alignment::HorizontalAlignment, style::TextBoxStyleBuilder, TextBox};

struct MenuLine<'a, 'b, C, I> {
    pub bounds: Margin<Rectangle>,
    pub text_style: MonoTextStyle<'a, C>,
    pub item: &'b I,
}

impl<C, I> Drawable for MenuLine<'_, '_, C, I>
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
