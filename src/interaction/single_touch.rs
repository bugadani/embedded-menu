use crate::interaction::{InteractionController, InteractionType};

#[derive(Default, Debug)]
pub struct State {
    interaction_time: u32,
    was_released: bool,
    interacted_before_release: bool,
}

/// Single touch navigation in hierarchical lists
///
/// Short press: select next item
/// Long press: activate current item
#[derive(Clone, Copy)]
pub struct SingleTouch {
    /// Does not display short presses on the selection indicator.
    pub ignore_time: u32,

    /// Ignores touches shorter than this many update periods.
    pub debounce_time: u32,

    /// Detects long presses after this many update periods.
    pub max_time: u32,
}

impl InteractionController for SingleTouch {
    type Input = bool;
    type State = State;

    fn fill_area_width(&self, state: &Self::State, max: u32) -> u32 {
        if self.ignore_time <= state.interaction_time && state.interaction_time < self.max_time {
            // Draw indicator bar
            let time = (state.interaction_time - self.ignore_time) as f32
                / ((self.max_time - self.ignore_time) as f32 * 0.9);

            ((time * (max - 1) as f32) as u32).max(0)
        } else {
            // Don't draw anything
            0
        }
    }

    fn update(&self, state: &mut Self::State, action: Self::Input) -> Option<InteractionType> {
        if !state.was_released {
            if action {
                return None;
            }
            state.was_released = true;
        }

        if action {
            state.interaction_time = state.interaction_time.saturating_add(1);
            if state.interaction_time < self.max_time {
                None
            } else {
                state.interacted_before_release = true;
                state.interaction_time = 0;
                Some(InteractionType::Select)
            }
        } else {
            let time = core::mem::replace(&mut state.interaction_time, 0);

            if self.debounce_time < time && time < self.max_time && !state.interacted_before_release
            {
                Some(InteractionType::Next)
            } else {
                // Already interacted before releasing, ignore and reset.
                state.interacted_before_release = false;
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::interaction::{single_touch::SingleTouch, InteractionController, InteractionType};

    #[test]
    fn test_interaction() {
        // ignore 1 long pulses, accept 2-4 as short press, 5- as long press
        let controller = SingleTouch {
            ignore_time: 1,
            debounce_time: 1,
            max_time: 5,
        };

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
            &[
                (1, false, None),
                (2, true, None),
                (1, false, Some(InteractionType::Next)),
            ],
            &[
                (1, false, None),
                (3, true, None),
                (1, false, Some(InteractionType::Next)),
            ],
            // long pulse NOT recognised as Select event on falling edge
            &[
                (1, false, None),
                (4, true, None),
                (1, false, Some(InteractionType::Next)),
            ],
            // long pulse recognised as Select event immediately
            &[
                (1, false, None),
                (4, true, None),
                (1, true, Some(InteractionType::Select)),
                (4, true, None),
                (1, true, Some(InteractionType::Select)),
                (4, true, None),
                (1, false, None),
            ],
        ];

        for (row, &inputs) in expectations.iter().enumerate() {
            let mut controller_state = Default::default();

            let mut sample = 0;
            for (repeat, input, expectation) in inputs.iter() {
                for _ in 0..*repeat {
                    let ret = controller.update(&mut controller_state, *input);

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
