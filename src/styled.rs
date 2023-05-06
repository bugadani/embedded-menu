use embedded_graphics::{
    prelude::{DrawTarget, PixelColor},
    primitives::StyledDrawable,
};
use embedded_layout::{
    object_chain::ChainElement,
    prelude::{Chain, Link},
};

use crate::{
    interaction::InteractionController,
    selection_indicator::{style::IndicatorStyle, SelectionIndicatorController},
    MenuStyle,
};

pub trait StyledMenuItem<C, S, IT, P>:
    StyledDrawable<MenuStyle<C, S, IT, P>, Color = C, Output = ()>
where
    C: PixelColor,
    S: IndicatorStyle,
    IT: InteractionController,
    P: SelectionIndicatorController,
{
}
impl<T, C, S, IT, P> StyledMenuItem<C, S, IT, P> for T
where
    T: StyledDrawable<MenuStyle<C, S, IT, P>, Color = C, Output = ()>,
    C: PixelColor,
    S: IndicatorStyle,
    IT: InteractionController,
    P: SelectionIndicatorController,
{
}

impl<C, S, V, VC, IT, P> StyledDrawable<MenuStyle<C, S, IT, P>> for Link<V, VC>
where
    C: PixelColor,
    V: StyledMenuItem<C, S, IT, P>,
    VC: ChainElement + StyledMenuItem<C, S, IT, P>,
    S: IndicatorStyle,
    IT: InteractionController,
    P: SelectionIndicatorController,
{
    type Color = C;
    type Output = ();

    #[inline]
    fn draw_styled<D>(
        &self,
        style: &MenuStyle<Self::Color, S, IT, P>,
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

impl<C, S, V, IT, P> StyledDrawable<MenuStyle<C, S, IT, P>> for Chain<V>
where
    C: PixelColor,
    V: StyledMenuItem<C, S, IT, P>,
    S: IndicatorStyle,
    IT: InteractionController,
    P: SelectionIndicatorController,
{
    type Color = C;
    type Output = ();

    #[inline]
    fn draw_styled<D>(
        &self,
        style: &MenuStyle<Self::Color, S, IT, P>,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.object.draw_styled(style, display)?;
        Ok(())
    }
}
