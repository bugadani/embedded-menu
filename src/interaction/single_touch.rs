use crate::interaction::{InteractionController, InteractionType};

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::BinaryColor,
    prelude::{Point, Size},
    primitives::{Primitive, PrimitiveStyle, Rectangle},
    Drawable,
};

pub struct SingleTouch {
    interaction_time: u32,
    ignore_time: u32,
    max_time: u32,
}

/// Single touch navigation in hierarchical lists
///
/// Short press: select next item
/// Long press: activate current item
///             Holding the input does not cause the current item to fire repeatedly
///
/// Requires a `back` element.
impl SingleTouch {
    /// `ignore`: Ignore pulses with at most this many active samples
    /// `max`: Detect pulses with this many active samples as `Select`
    pub fn new(ignore: u32, max: u32) -> Self {
        Self {
            interaction_time: 0,
            ignore_time: ignore,
            max_time: max,
        }
    }
}

impl InteractionController for SingleTouch {
    type Input = bool;

    fn reset(&mut self) {
        self.interaction_time = 0;
    }

    fn fill_area_width(&self, max: u32) -> u32 {
        if self.ignore_time <= self.interaction_time && self.interaction_time < self.max_time {
            // Draw indicator bar
            let time = (self.interaction_time - self.ignore_time) as f32
                / ((self.max_time - self.ignore_time) as f32 * 0.9);

            ((time * (max - 1) as f32) as u32).max(1)
        } else {
            // Don't draw anything
            1
        }
    }

    fn update(&mut self, action: Self::Input) -> Option<InteractionType> {
        if action {
            if self.interaction_time < self.max_time {
                self.interaction_time = self.interaction_time.saturating_add(1);

                if self.interaction_time == self.max_time {
                    Some(InteractionType::Select)
                } else {
                    None
                }
            } else {
                // holding input after event has fired
                None
            }
        } else {
            let time = core::mem::replace(&mut self.interaction_time, 0);
            if time <= self.ignore_time {
                None
            } else if time < self.max_time {
                Some(InteractionType::Next)
            } else {
                // Already interacted
                None
            }
        }
    }
}

impl Drawable for SingleTouch {
    type Color = BinaryColor;
    type Output = ();

    fn draw<D: DrawTarget<Color = BinaryColor>>(&self, display: &mut D) -> Result<(), D::Error> {
        Rectangle::new(
            Point::zero(),
            Size::new(
                self.fill_area_width(display.bounding_box().size.width),
                display.bounding_box().size.height,
            ),
        )
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(display)
    }
}

#[cfg(test)]
mod test {
    use crate::interaction::{single_touch::SingleTouch, InteractionController, InteractionType};

    #[test]
    fn test_interaction() {
        // ignore 1 long pulses, accept 2-4 as short press, 5- as long press
        let mut controller = SingleTouch::new(1, 5);

        let expectations: [&[_]; 6] = [
            &[(5, false, None)],
            // repeated short pulse ignored
            &[
                (1, true, None),
                (1, false, None),
                (1, true, None),
                (1, false, None),
                (1, true, None),
                (1, false, None),
            ],
            // longer pulse recongised as Next event on falling edge
            &[(2, true, None), (1, false, Some(InteractionType::Next))],
            &[(3, true, None), (1, false, Some(InteractionType::Next))],
            // long pulse NOT recognised as Select event on falling edge
            &[(4, true, None), (1, false, Some(InteractionType::Next))],
            // long pulse recognised as Select event immediately
            &[
                (4, true, None),
                (1, true, Some(InteractionType::Select)),
                (10, true, None),
                (1, false, None),
            ],
        ];

        for (row, &inputs) in expectations.iter().enumerate() {
            controller.reset();

            let mut sample = 0;
            for (repeat, input, expectation) in inputs.iter() {
                for _ in 0..*repeat {
                    let ret = controller.update(*input);

                    assert_eq!(
                        ret, *expectation,
                        "Mismatch at row {}, sample {}",
                        row, sample
                    );
                    sample += 1;
                }
            }
        }
    }
}
