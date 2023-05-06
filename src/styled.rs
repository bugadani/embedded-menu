use embedded_graphics::{
    prelude::{DrawTarget, PixelColor},
    primitives::StyledDrawable,
};
use embedded_layout::{
    object_chain::ChainElement,
    prelude::{Chain, Link},
};

use crate::{selection_indicator::style::IndicatorStyle, MenuStyle};

pub trait StyledMenuItem<C, S>: StyledDrawable<MenuStyle<C, S>, Color = C, Output = ()>
where
    C: PixelColor,
    S: IndicatorStyle,
{
}
impl<T, C, S> StyledMenuItem<C, S> for T
where
    T: StyledDrawable<MenuStyle<C, S>, Color = C, Output = ()>,
    C: PixelColor,
    S: IndicatorStyle,
{
}

impl<C, S, V, VC> StyledDrawable<MenuStyle<C, S>> for Link<V, VC>
where
    C: PixelColor,
    V: StyledMenuItem<C, S>,
    VC: ChainElement + StyledMenuItem<C, S>,
    S: IndicatorStyle,
{
    type Color = C;
    type Output = ();

    #[inline]
    fn draw_styled<D>(
        &self,
        style: &MenuStyle<Self::Color, S>,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.object.draw_styled(style, display)?;
        self.parent.draw_styled(style, display)?;

        Ok(())
    }
}

impl<C, S, V> StyledDrawable<MenuStyle<C, S>> for Chain<V>
where
    C: PixelColor,
    V: StyledMenuItem<C, S>,
    S: IndicatorStyle,
{
    type Color = C;
    type Output = ();

    #[inline]
    fn draw_styled<D>(
        &self,
        style: &MenuStyle<Self::Color, S>,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.object.draw_styled(style, display)?;
        Ok(())
    }
}
