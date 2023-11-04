use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point},
    primitives::Rectangle,
};
use embedded_layout::View;

use crate::{
    interaction::InputAdapterSource,
    items::MenuLine,
    selection_indicator::{style::IndicatorStyle, SelectionIndicatorController},
    theme::Theme,
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

impl<T, R> MenuItem<R> for SectionTitle<T>
where
    T: AsRef<str>,
{
    fn value_of(&self) -> R {
        unreachable!("Selected a non-selectable menu item")
    }

    fn interact(&mut self) -> R {
        unreachable!("Selected a non-selectable menu item")
    }

    fn selectable(&self) -> bool {
        false
    }

    fn set_style<S, IT, P, C>(&mut self, style: &MenuStyle<S, IT, P, R, C>)
    where
        S: IndicatorStyle<Theme = C>,
        IT: InputAdapterSource<R>,
        P: SelectionIndicatorController,
        C: Theme,
    {
        self.line = MenuLine::new("", style);
    }

    fn draw_styled<S, IT, P, D, C>(
        &self,
        style: &MenuStyle<S, IT, P, R, C>,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        S: IndicatorStyle<Theme = C>,
        IT: InputAdapterSource<R>,
        P: SelectionIndicatorController,
        D: DrawTarget<Color = BinaryColor>,
        C: Theme,
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
