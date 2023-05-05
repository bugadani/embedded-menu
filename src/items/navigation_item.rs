use crate::{
    items::MenuLine,
    margin::{Margin, MarginExt},
    MenuEvent, MenuItemTrait,
};

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::{PixelColor, Point, Size},
    primitives::Rectangle,
    Drawable,
};
use embedded_layout::prelude::*;

pub struct NavigationItemData<'a, R: Copy> {
    title_text: &'a str,
    details: &'a str,
    return_value: R,
    marker: &'a str,
}

pub struct NavigationItem<'a, R: Copy, C: PixelColor> {
    data: NavigationItemData<'a, R>,
    style: MonoTextStyle<'a, C>,
    bounds: Margin<Rectangle>,
}

impl<'a, R: Copy, C: PixelColor> NavigationItem<'a, R, C> {
    pub fn new(marker: &'a str, title: &'a str, details: &'a str, value: R, color: C) -> Self {
        let style = MonoTextStyle::<C>::new(&FONT_6X10, color);

        Self {
            data: NavigationItemData {
                marker,
                title_text: title,
                details,
                return_value: value,
            },
            style,
            bounds: Rectangle::new(
                Point::zero(),
                Size::new(1, style.font.character_size.height),
            )
            .with_margin(0, 0, -1, 1),
        }
    }
}

impl<'a, R: Copy, C: PixelColor> MenuItemTrait<R> for NavigationItem<'a, R, C> {
    fn interact(&mut self) -> MenuEvent<R> {
        MenuEvent::NavigationEvent(self.data.return_value)
    }

    fn title(&self) -> &str {
        self.data.title_text
    }

    fn details(&self) -> &str {
        self.data.details
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
        let menu_line = MenuLine {
            title: self.data.title_text,
            value: self.data.marker,
            bounds: self.bounds,
            text_style: self.style,
        };

        menu_line.draw(display)
    }
}
