use crate::interaction::{InputAdapter, InputState, InteractionType};

#[derive(Default, Debug, Clone, Copy)]
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

fn progress(current: u32, target: u32) -> u8 {
    if current < target {
        let time = current as f32 / (target as f32 * 0.9);

        ((time * (255 - 1) as f32) as u32).max(0) as u8
    } else {
        0
    }
}
impl InputAdapter for SingleTouch {
    type Input = bool;
    type State = State;

    fn handle_input(&self, state: &mut Self::State, action: Self::Input) -> InputState {
        if !state.was_released {
            if action {
                return InputState::Idle;
            }
            state.was_released = true;
        }

        if action {
            state.interaction_time = state.interaction_time.saturating_add(1);
            if state.interaction_time <= self.ignore_time {
                InputState::Idle
            } else if state.interaction_time < self.max_time {
                InputState::InProgress(progress(
                    state.interaction_time - self.ignore_time,
                    self.max_time - self.ignore_time,
                ))
            } else {
                state.interacted_before_release = true;
                state.interaction_time = 0;
                InputState::Active(InteractionType::Select)
            }
        } else {
            let time = core::mem::replace(&mut state.interaction_time, 0);

            if self.debounce_time < time && time < self.max_time && !state.interacted_before_release
            {
                InputState::Active(InteractionType::Next)
            } else {
                // Already interacted before releasing, ignore and reset.
                state.interacted_before_release = false;
                InputState::Idle
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::interaction::{
        single_touch::SingleTouch, InputAdapter, InputState, InteractionType,
    };

    #[test]
    fn test_interaction() {
        // ignore 1 long pulses, accept 2-4 as short press, 5- as long press
        let controller = SingleTouch {
            ignore_time: 1,
            debounce_time: 1,
            max_time: 5,
        };

        let expectations: [&[_]; 6] = [
            &[
                (false, InputState::Idle),
                (false, InputState::Idle),
                (false, InputState::Idle),
                (false, InputState::Idle),
                (false, InputState::Idle),
                (false, InputState::Idle),
            ],
            // repeated short pulse ignored
            &[
                (true, InputState::Idle),
                (false, InputState::Idle),
                (true, InputState::Idle),
                (false, InputState::Idle),
                (true, InputState::Idle),
                (false, InputState::Idle),
            ],
            // longer pulse recongised as Next event on falling edge
            &[
                (false, InputState::Idle),
                (true, InputState::Idle),
                (true, InputState::InProgress(70)),
                (false, InputState::Active(InteractionType::Next)),
            ],
            &[
                (false, InputState::Idle),
                (true, InputState::Idle),
                (true, InputState::InProgress(70)),
                (true, InputState::InProgress(141)),
                (false, InputState::Active(InteractionType::Next)),
            ],
            // long pulse NOT recognised as Select event on falling edge
            &[
                (false, InputState::Idle),
                (true, InputState::Idle),
                (true, InputState::InProgress(70)),
                (true, InputState::InProgress(141)),
                (true, InputState::InProgress(211)),
                (false, InputState::Active(InteractionType::Next)),
            ],
            // long pulse recognised as Select event immediately
            &[
                (false, InputState::Idle),
                (true, InputState::Idle),
                (true, InputState::InProgress(70)),
                (true, InputState::InProgress(141)),
                (true, InputState::InProgress(211)),
                (true, InputState::Active(InteractionType::Select)),
                (true, InputState::Idle),
                (true, InputState::InProgress(70)),
                (true, InputState::InProgress(141)),
                (true, InputState::InProgress(211)),
                (true, InputState::Active(InteractionType::Select)),
                (true, InputState::Idle),
                (true, InputState::InProgress(70)),
                (true, InputState::InProgress(141)),
                (true, InputState::InProgress(211)),
                (false, InputState::Idle),
            ],
        ];

        for (row, &inputs) in expectations.iter().enumerate() {
            let mut controller_state = Default::default();

            for (sample, (input, expectation)) in inputs.iter().enumerate() {
                let ret = controller.handle_input(&mut controller_state, *input);

                assert_eq!(
                    ret, *expectation,
                    "Mismatch at row {}, sample {}",
                    row, sample
                );
            }
        }
    }
}
