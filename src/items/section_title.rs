use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::{DrawTarget, PixelColor, Point},
    primitives::Rectangle,
};
use embedded_layout::View;

use crate::{
    interaction::InputAdapterSource,
    items::MenuLine,
    selection_indicator::{style::IndicatorStyle, SelectionIndicatorController},
    Marker, MenuItem, MenuStyle,
};

pub struct SectionTitle<T, R>
where
    T: AsRef<str>,
{
    title_text: T,
    return_value: R,
    line: MenuLine,
}

impl<T, R> Marker for SectionTitle<T, R> where T: AsRef<str> {}

impl<T, R> MenuItem<R> for SectionTitle<T, R>
where
    T: AsRef<str>,
    R: Copy,
{
    fn interact(&mut self) -> R {
        self.return_value
    }

    fn title(&self) -> &str {
        self.title_text.as_ref()
    }

    fn details(&self) -> &str {
        ""
    }

    fn value(&self) -> &str {
        ""
    }

    fn selectable(&self) -> bool {
        false
    }

    fn set_style<C, S, IT, P>(&mut self, style: &MenuStyle<C, S, IT, P, R>)
    where
        C: PixelColor,
        S: IndicatorStyle,
        IT: InputAdapterSource<R>,
        P: SelectionIndicatorController,
    {
        self.line = MenuLine::new("", style);
    }

    fn draw_styled<C, S, IT, P, DIS>(
        &self,
        style: &MenuStyle<C, S, IT, P, R>,
        display: &mut DIS,
    ) -> Result<(), DIS::Error>
    where
        C: PixelColor + From<Rgb888>,
        S: IndicatorStyle,
        IT: InputAdapterSource<R>,
        P: SelectionIndicatorController,
        DIS: DrawTarget<Color = C>,
    {
        self.line
            .draw_styled(self.title_text.as_ref(), "", style, display)
    }
}

impl<T> SectionTitle<T, ()>
where
    T: AsRef<str>,
{
    pub fn new(title: T) -> Self {
        Self {
            title_text: title,
            return_value: (),
            line: MenuLine::empty(),
        }
    }
}

impl<T, R> View for SectionTitle<T, R>
where
    T: AsRef<str>,
{
    fn translate_impl(&mut self, by: Point) {
        self.line.translate_mut(by);
    }

    fn bounds(&self) -> Rectangle {
        self.line.bounds()
    }
}
