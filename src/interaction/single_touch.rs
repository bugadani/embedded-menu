use crate::interaction::{InteractionController, InteractionType};
use embedded_layout::prelude::*;

use embedded_graphics::{
    pixelcolor::BinaryColor, primitives::Rectangle, style::PrimitiveStyle, DrawTarget,
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

    fn update(&mut self, action: Self::Input) -> InteractionType {
        if action {
            if self.interaction_time < self.max_time {
                self.interaction_time = self.interaction_time.saturating_add(1);

                if self.interaction_time == self.max_time {
                    InteractionType::Select
                } else {
                    InteractionType::Nothing
                }
            } else {
                // holding input after event has fired
                InteractionType::Nothing
            }
        } else {
            let time = core::mem::replace(&mut self.interaction_time, 0);
            if time <= self.ignore_time {
                InteractionType::Nothing
            } else if time < self.max_time {
                InteractionType::Next
            } else {
                // Already interacted
                InteractionType::Nothing
            }
        }
    }
}

impl Drawable<BinaryColor> for &SingleTouch {
    fn draw<D: DrawTarget<BinaryColor>>(self, display: &mut D) -> Result<(), D::Error> {
        let width =
            if self.ignore_time <= self.interaction_time && self.interaction_time < self.max_time {
                // Draw indicator bar
                let time = (self.interaction_time - self.ignore_time) as f32
                    / ((self.max_time - self.ignore_time) as f32 * 0.9);

                ((time * (display.size().width - 1) as f32) as u32).max(1)
            } else {
                // Don't draw anything
                1
            };

        Rectangle::with_size(Point::new(0, 0), Size::new(width, display.size().height))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(display)
    }
}

/*
impl Drawable<ExtendedBinaryColor> for &SingleTouch {
    fn draw<D: DrawTarget<ExtendedBinaryColor>>(self, display: &mut D) -> Result<(), D::Error> {
        let width =
            if self.ignore_time <= self.interaction_time && self.interaction_time < self.max_time {
                // Draw indicator bar
                let time = (self.interaction_time - self.ignore_time) as f32
                    / ((self.max_time - self.ignore_time) as f32 * 0.9);

                ((time * (display.size().width - 1) as f32) as u32).max(1)
            } else {
                // Don't draw anything
                1
            };

        Rectangle::with_size(Point::new(0, 0), Size::new(width, display.size().height))
            .into_styled(PrimitiveStyle::with_fill(ExtendedBinaryColor::Invert))
            .draw(display)
    }
}
*/

#[cfg(test)]
mod test {
    use crate::interaction::{single_touch::SingleTouch, InteractionController, InteractionType};

    #[test]
    fn test_interaction() {
        // ignore 1 long pulses, accept 2-4 as short press, 5- as long press
        let mut controller = SingleTouch::new(1, 5);

        let expectations: [&[(usize, bool, InteractionType)]; 6] = [
            &[(5, false, InteractionType::Nothing)],
            // repeated short pulse ignored
            &[
                (1, true, InteractionType::Nothing),
                (1, false, InteractionType::Nothing),
                (1, true, InteractionType::Nothing),
                (1, false, InteractionType::Nothing),
                (1, true, InteractionType::Nothing),
                (1, false, InteractionType::Nothing),
            ],
            // longer pulse recongised as Next event on falling edge
            &[
                (2, true, InteractionType::Nothing),
                (1, false, InteractionType::Next),
            ],
            &[
                (3, true, InteractionType::Nothing),
                (1, false, InteractionType::Next),
            ],
            // long pulse NOT recognised as Select event on falling edge
            &[
                (4, true, InteractionType::Nothing),
                (1, false, InteractionType::Next),
            ],
            // long pulse recognised as Select event immediately
            &[
                (4, true, InteractionType::Nothing),
                (1, true, InteractionType::Select),
                (10, true, InteractionType::Nothing),
                (1, false, InteractionType::Nothing),
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
