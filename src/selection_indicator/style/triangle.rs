use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, PixelColor, Point, Size},
    primitives::{ContainsPoint, Primitive, PrimitiveStyle, Rectangle, Triangle as TriangleShape},
    transform::Transform,
    Drawable,
};
use embedded_layout::prelude::{horizontal::LeftToRight, vertical::Center, Align};

use crate::{
    interaction::InputState,
    selection_indicator::{
        style::{interpolate, IndicatorStyle},
        Insets,
    },
    theme::Theme,
};

#[derive(Clone, Copy)]
pub struct Triangle<T = BinaryColor> {
    theme: T,
}

impl<T> IndicatorStyle for Triangle<T>
where
    T: Theme,
{
    type Shape = Arrow<T::Color>;
    type State = ();
    type Theme = T;

    fn padding(&self, _state: &Self::State, height: i32) -> Insets {
        Insets {
            left: height / 2 + 1,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }

    fn shape(&self, state: &Self::State, bounds: Rectangle, fill_width: u32) -> Self::Shape {
        Arrow::new(bounds, fill_width, self.color(state))
    }

    fn color(&self, _state: &Self::State) -> <Self::Theme as Theme>::Color {
        self.theme.selection_color()
    }

    fn draw<D>(
        &self,
        state: &Self::State,
        input_state: InputState,
        display: &mut D,
    ) -> Result<Self::Shape, D::Error>
    where
        D: DrawTarget<Color = <Self::Theme as Theme>::Color>,
    {
        let display_area = display.bounding_box();

        let fill_width = if let InputState::InProgress(progress) = input_state {
            interpolate(progress as u32, 0, 255, 0, display_area.size.width)
        } else {
            0
        };

        let shape = self.shape(state, display_area, fill_width);

        // todo: Can't seem to get the `Into`/`From` trait to work nicely here
        let draw_shape: Arrow<<D as DrawTarget>::Color> = Arrow {
            body: shape.body,
            tip: shape.tip,
            color: self.theme.selection_color(),
        };

        draw_shape.draw(display)?;
        Ok(shape)
    }
}

#[derive(Clone, Copy)]
pub struct Arrow<C = BinaryColor> {
    body: Rectangle,
    tip: TriangleShape,
    color: C,
}

const SHRINK: i32 = 1;

impl<C> Arrow<C> {
    pub fn new(bounds: Rectangle, fill_width: u32, color: C) -> Self {
        let body = Rectangle::new(bounds.top_left, Size::new(fill_width, bounds.size.height));

        let tip = TriangleShape::new(
            Point::new(0, SHRINK),
            Point::new(0, Self::tip_width(bounds)),
            Point::new(
                bounds.size.height as i32 / 2 - SHRINK,
                bounds.size.height as i32 / 2,
            ),
        )
        .align_to(&body, LeftToRight, Center)
        // e-layout doesn't align well to 0 area rectangles
        .translate(Point::new(if body.is_zero_sized() { -1 } else { 0 }, 0));

        Self { body, tip, color }
    }

    pub fn tip_width(bounds: Rectangle) -> i32 {
        bounds.size.height as i32 - 1 - SHRINK
    }
}

impl<C> ContainsPoint for Arrow<C> {
    fn contains(&self, point: Point) -> bool {
        self.body.contains(point) || self.tip.contains(point)
    }
}

impl<C> Transform for Arrow<C>
where
    C: Copy,
{
    fn translate(&self, by: Point) -> Self {
        Self {
            body: self.body.translate(by),
            tip: self.tip.translate(by),
            color: self.color,
        }
    }

    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.body.translate_mut(by);
        self.tip.translate_mut(by);
        self
    }
}

impl<C> Drawable for Arrow<C>
where
    C: PixelColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let style = PrimitiveStyle::with_fill(self.color);

        self.body.into_styled(style).draw(target)?;
        self.tip.into_styled(style).draw(target)?;

        Ok(())
    }
}
