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

pub struct SectionTitle<T>
where
    T: AsRef<str>,
{
    title_text: T,
    line: MenuLine,
}

impl<T> Marker for SectionTitle<T> where T: AsRef<str> {}

impl<T> MenuItem<()> for SectionTitle<T>
where
    T: AsRef<str>,
{
    fn interact(&mut self) {}

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

    fn set_style<C, S, IT, P>(&mut self, style: &MenuStyle<C, S, IT, P, ()>)
    where
        C: PixelColor,
        S: IndicatorStyle,
        IT: InputAdapterSource<()>,
        P: SelectionIndicatorController,
    {
        self.line = MenuLine::new("", style);
    }

    fn draw_styled<C, S, IT, P, DIS>(
        &self,
        style: &MenuStyle<C, S, IT, P, ()>,
        display: &mut DIS,
    ) -> Result<(), DIS::Error>
    where
        C: PixelColor + From<Rgb888>,
        S: IndicatorStyle,
        IT: InputAdapterSource<()>,
        P: SelectionIndicatorController,
        DIS: DrawTarget<Color = C>,
    {
        self.line
            .draw_styled(self.title_text.as_ref(), "", style, display)
    }
}

impl<T> SectionTitle<T>
where
    T: AsRef<str>,
{
    pub fn new(title: T) -> Self {
        Self {
            title_text: title,
            line: MenuLine::empty(),
        }
    }
}

impl<T> View for SectionTitle<T>
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
