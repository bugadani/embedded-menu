use embedded_graphics::{
    prelude::{DrawTarget, PixelColor},
    primitives::StyledDrawable,
};
use embedded_layout::{
    object_chain::ChainElement,
    prelude::{Chain, Link},
};

use crate::MenuStyle;

pub trait StyledMenuItem<C: PixelColor>:
    StyledDrawable<MenuStyle<C>, Color = C, Output = ()>
{
}
impl<T, C: PixelColor> StyledMenuItem<C> for T where
    T: StyledDrawable<MenuStyle<C>, Color = C, Output = ()>
{
}

impl<C, V, VC> StyledDrawable<MenuStyle<C>> for Link<V, VC>
where
    C: PixelColor,
    V: StyledMenuItem<C>,
    VC: ChainElement + StyledMenuItem<C>,
{
    type Color = C;
    type Output = ();

    #[inline]
    fn draw_styled<D>(
        &self,
        style: &MenuStyle<Self::Color>,
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

impl<C, V> StyledDrawable<MenuStyle<C>> for Chain<V>
where
    C: PixelColor,
    V: StyledMenuItem<C>,
{
    type Color = C;
    type Output = ();

    #[inline]
    fn draw_styled<D>(
        &self,
        style: &MenuStyle<Self::Color>,
        display: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.object.draw_styled(style, display)?;
        Ok(())
    }
}
