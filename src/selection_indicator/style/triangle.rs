use embedded_graphics::{
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
pub struct Triangle;

impl IndicatorStyle for Triangle {
    type Shape = Arrow;
    type State = ();

    fn padding(&self, _state: &Self::State, height: i32) -> Insets {
        Insets {
            left: height / 2 + 1,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }

    fn shape(&self, _state: &Self::State, bounds: Rectangle, fill_width: u32) -> Self::Shape {
        Arrow::new(bounds, fill_width)
    }

    fn draw<T, D>(
        &self,
        state: &Self::State,
        input_state: InputState,
        theme: &T,
        display: &mut D,
    ) -> Result<Self::Shape, D::Error>
    where
        T: Theme,
        D: DrawTarget<Color = T::Color>,
    {
        let display_area = display.bounding_box();

        let fill_width = if let InputState::InProgress(progress) = input_state {
            interpolate(progress as u32, 0, 255, 0, display_area.size.width)
        } else {
            0
        };

        let shape = self.shape(state, display_area, fill_width);

        shape.draw(theme.selection_color(), display)?;

        Ok(shape)
    }
}

#[derive(Clone, Copy)]
pub struct Arrow {
    body: Rectangle,
    tip: TriangleShape,
}

const SHRINK: i32 = 1;

impl Arrow {
    pub fn new(bounds: Rectangle, fill_width: u32) -> Self {
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

        Self { body, tip }
    }

    pub fn tip_width(bounds: Rectangle) -> i32 {
        bounds.size.height as i32 - 1 - SHRINK
    }

    pub fn draw<D, C>(&self, color: C, target: &mut D) -> Result<(), D::Error>
    where
        C: PixelColor,
        D: DrawTarget<Color = C>,
    {
        let style = PrimitiveStyle::with_fill(color);

        self.body.into_styled(style).draw(target)?;
        self.tip.into_styled(style).draw(target)?;

        Ok(())
    }
}

impl ContainsPoint for Arrow {
    fn contains(&self, point: Point) -> bool {
        self.body.contains(point) || self.tip.contains(point)
    }
}

impl Transform for Arrow {
    fn translate(&self, by: Point) -> Self {
        Self {
            body: self.body.translate(by),
            tip: self.tip.translate(by),
        }
    }

    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.body.translate_mut(by);
        self.tip.translate_mut(by);
        self
    }
}
