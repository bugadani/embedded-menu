use embedded_graphics::{
    prelude::{DrawTarget, PixelColor},
    primitives::StyledDrawable,
};
use embedded_layout::{
    object_chain::ChainElement,
    prelude::{Chain, Link},
};

use crate::{
    interaction::InteractionController, selection_indicator::style::IndicatorStyle, MenuStyle,
};

pub trait StyledMenuItem<C, S, IT>:
    StyledDrawable<MenuStyle<C, S, IT>, Color = C, Output = ()>
where
    C: PixelColor,
    S: IndicatorStyle,
    IT: InteractionController,
{
}
impl<T, C, S, IT> StyledMenuItem<C, S, IT> for T
where
    T: StyledDrawable<MenuStyle<C, S, IT>, Color = C, Output = ()>,
    C: PixelColor,
    S: IndicatorStyle,
    IT: InteractionController,
{
}

impl<C, S, V, VC, IT> StyledDrawable<MenuStyle<C, S, IT>> for Link<V, VC>
where
    C: PixelColor,
    V: StyledMenuItem<C, S, IT>,
    VC: ChainElement + StyledMenuItem<C, S, IT>,
    S: IndicatorStyle,
    IT: InteractionController,
{
    type Color = C;
    type Output = ();

    #[inline]
    fn draw_styled<D>(
        &self,
        style: &MenuStyle<Self::Color, S, IT>,
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

impl<C, S, V, IT> StyledDrawable<MenuStyle<C, S, IT>> for Chain<V>
where
    C: PixelColor,
    V: StyledMenuItem<C, S, IT>,
    S: IndicatorStyle,
    IT: InteractionController,
{
    type Color = C;
    type Output = ();

    #[inline]
    fn draw_styled<D>(
        &self,
        style: &MenuStyle<Self::Color, S, IT>,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.object.draw_styled(style, display)?;
        Ok(())
    }
}
