use crate::{Margin, MarginExt, MenuEvent, MenuItemTrait};

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::{PixelColor, Point},
    primitives::Rectangle,
    text::{renderer::TextRenderer, Baseline},
    Drawable,
};
use embedded_layout::prelude::*;
use embedded_text::TextBox;

pub struct NavigationItem<'a, R: Copy, C: PixelColor> {
    style: MonoTextStyle<'a, C>,
    marker: &'a str,
    title_text: &'a str,
    details: &'a str,
    return_value: R,
    bounds: Margin<Rectangle>,
}

impl<'a, R: Copy, C: PixelColor> NavigationItem<'a, R, C> {
    pub fn new(marker: &'a str, title: &'a str, details: &'a str, value: R, color: C) -> Self {
        let style = MonoTextStyle::<C>::new(&FONT_6X10, color);

        Self {
            marker,
            style,
            title_text: title,
            details,
            return_value: value,
            bounds: style
                .measure_string(title, Point::zero(), Baseline::Top)
                .bounding_box
                .with_margin(1, 0, 0, 1),
        }
    }
}

impl<'a, R: Copy, C: PixelColor> MenuItemTrait<R> for NavigationItem<'a, R, C> {
    fn interact(&mut self) -> MenuEvent<R> {
        MenuEvent::NavigationEvent(self.return_value)
    }

    fn title(&self) -> &str {
        self.title_text
    }

    fn details(&self) -> &str {
        self.details
    }
}

impl<'a, R: Copy, C: PixelColor> View for NavigationItem<'a, R, C> {
    fn translate_impl(&mut self, by: Point) {
        self.bounds.translate_mut(by);
    }

    fn bounds(&self) -> Rectangle {
        self.bounds.bounds()
    }
}

impl<'a, R, C> Drawable for NavigationItem<'a, R, C>
where
    R: Copy,
    C: PixelColor + From<Rgb888>,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let text_bounds = self.bounds();
        let display_area = display.bounding_box();

        if text_bounds.intersection(&display_area).is_zero_sized() {
            return Ok(());
        }

        let inner_bounds = self.bounds.inner().bounds();

        TextBox::new(self.title_text, inner_bounds, self.style).draw(display)?;

        TextBox::new(
            self.marker,
            Rectangle::new(inner_bounds.top_left, inner_bounds.size()),
            self.style,
        )
        .align_to(&display_area, horizontal::Right, vertical::NoAlignment)
        .draw(display)?;

        Ok(())
    }
}
