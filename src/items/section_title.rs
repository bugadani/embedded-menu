use embedded_graphics::{
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
    line: MenuLine,
    _r: core::marker::PhantomData<R>,
}

impl<T, R> Marker for SectionTitle<T, R> where T: AsRef<str> {}

impl<T, R> MenuItem<R> for SectionTitle<T, R>
where
    T: AsRef<str>,
    R: Default,
{
    fn value_of(&self) -> R {
        R::default()
    }

    fn interact(&mut self) -> R {
        R::default()
    }

    fn selectable(&self) -> bool {
        false
    }

    fn set_style<S, IT, P, C>(&mut self, style: &MenuStyle<S, IT, P, R, C>)
    where
        S: IndicatorStyle,
        IT: InputAdapterSource<R>,
        P: SelectionIndicatorController,
        C: PixelColor,
    {
        self.line = MenuLine::new("", style);
    }

    fn draw_styled<S, IT, P, D, C>(
        &self,
        style: &MenuStyle<S, IT, P, R, C>,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        S: IndicatorStyle,
        IT: InputAdapterSource<R>,
        P: SelectionIndicatorController,
        D: DrawTarget<Color = C>,
        C: PixelColor + Default + 'static,
    {
        self.line
            .draw_styled(self.title_text.as_ref(), "", style, display)
    }
}

impl<T, R> SectionTitle<T, R>
where
    T: AsRef<str>,
{
    pub fn new(title: T) -> Self {
        Self {
            title_text: title,
            line: MenuLine::empty(),
            _r: core::marker::PhantomData,
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
