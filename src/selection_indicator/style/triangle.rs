use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point, Size},
    primitives::{ContainsPoint, Primitive, PrimitiveStyle, Rectangle, Triangle as TriangleShape},
    Drawable,
};
use embedded_layout::{
    prelude::{horizontal::LeftToRight, vertical::Center, Align},
    View,
};

use crate::{
    interaction::InputState,
    selection_indicator::{
        style::{interpolate, IndicatorStyle},
        Insets,
    },
};

#[derive(Clone, Copy)]
pub struct Triangle;

impl IndicatorStyle for Triangle {
    type Shape = Arrow;
    type State = ();

    fn padding(&self, _state: &Self::State, height: u32) -> Insets {
        Insets {
            left: height as i32 / 2 + 1,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }

    fn shape(&self, _state: &Self::State, bounds: Rectangle, fill_width: u32) -> Self::Shape {
        Arrow::new(bounds, fill_width)
    }

    fn draw<D>(
        &self,
        state: &Self::State,
        input_state: InputState,
        display: &mut D,
    ) -> Result<u32, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let display_area = display.bounding_box();

        let fill_width = if let InputState::InProgress(progress) = input_state {
            interpolate(progress as u32, 0, 255, 0, display_area.size.width)
        } else {
            0
        };

        self.shape(state, display_area, fill_width).draw(display)?;

        Ok(fill_width)
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
}

impl ContainsPoint for Arrow {
    fn contains(&self, point: Point) -> bool {
        self.body.contains(point) || self.tip.contains(point)
    }
}

impl View for Arrow {
    fn translate_impl(&mut self, by: Point) {
        self.body.translate_mut(by);
        self.tip.translate_mut(by);
    }

    fn bounds(&self) -> Rectangle {
        Rectangle::new(
            self.body.top_left,
            self.body.size + Size::new(self.tip.size().width, 0),
        )
    }
}

impl Drawable for Arrow {
    type Color = BinaryColor;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let style = PrimitiveStyle::with_fill(BinaryColor::On);

        self.body.into_styled(style).draw(target)?;
        self.tip.into_styled(style).draw(target)?;

        Ok(())
    }
}
