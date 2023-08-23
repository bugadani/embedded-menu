use core::marker::PhantomData;

use crate::{
    interaction::{Action, InputAdapter, InputAdapterSource, InputState, Interaction, Navigation},
    selection_indicator::style::interpolate,
};

#[derive(Default, Debug, Clone, Copy)]
pub struct State {
    interaction_time: u32,
    was_released: bool,
    repeated: bool,
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

impl<R> InputAdapterSource<R> for SingleTouch
where
    R: Copy,
{
    type InputAdapter = SingleTouchAdapter<R>;

    fn adapter(&self) -> Self::InputAdapter {
        SingleTouchAdapter {
            ignore_time: self.ignore_time,
            debounce_time: self.debounce_time,
            max_time: self.max_time,
            marker: PhantomData,
        }
    }
}

/// Single touch navigation in hierarchical lists
///
/// Short press: select next item
/// Long press: activate current item
#[derive(Clone, Copy)]
pub struct SingleTouchAdapter<R>
where
    R: Copy,
{
    ignore_time: u32,
    debounce_time: u32,
    max_time: u32,
    marker: PhantomData<R>,
}

impl<R> InputAdapter for SingleTouchAdapter<R>
where
    R: Copy,
{
    type Input = bool;
    type Value = R;
    type State = State;

    fn handle_input(
        &self,
        state: &mut Self::State,
        action: Self::Input,
    ) -> InputState<Self::Value> {
        if !state.was_released {
            if action {
                return InputState::Idle;
            }
            state.was_released = true;
        }

        if action {
            state.interaction_time = state.interaction_time.saturating_add(1);
            if state.interaction_time <= self.ignore_time && !state.repeated {
                InputState::Idle
            } else if state.interaction_time < self.max_time {
                let ignore_time = if state.repeated { 0 } else { self.ignore_time };
                InputState::InProgress(interpolate(
                    state.interaction_time - ignore_time,
                    0,
                    self.max_time - ignore_time,
                    0,
                    255,
                ) as u8)
            } else {
                state.repeated = true;
                state.interaction_time = 0;
                InputState::Active(Interaction::Action(Action::Select))
            }
        } else {
            let time = core::mem::replace(&mut state.interaction_time, 0);

            if self.debounce_time < time && time < self.max_time && !state.repeated {
                InputState::Active(Interaction::Navigation(Navigation::Next))
            } else {
                // Already interacted before releasing, ignore and reset.
                state.repeated = false;
                InputState::Idle
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::interaction::{
        single_touch::SingleTouch, Action, InputAdapter, InputAdapterSource, InputState,
        Interaction, Navigation,
    };

    #[test]
    fn test_interaction() {
        // ignore 1 long pulses, accept 2-4 as short press, 5- as long press
        let controller = SingleTouch {
            ignore_time: 1,
            debounce_time: 1,
            max_time: 5,
        }
        .adapter();

        let expectations: [&[(bool, InputState<()>)]; 6] = [
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
                (true, InputState::InProgress(63)),
                (
                    false,
                    InputState::Active(Interaction::Navigation(Navigation::Next)),
                ),
            ],
            &[
                (false, InputState::Idle),
                (true, InputState::Idle),
                (true, InputState::InProgress(63)),
                (true, InputState::InProgress(127)),
                (
                    false,
                    InputState::Active(Interaction::Navigation(Navigation::Next)),
                ),
            ],
            // long pulse NOT recognised as Select event on falling edge
            &[
                (false, InputState::Idle),
                (true, InputState::Idle),
                (true, InputState::InProgress(63)),
                (true, InputState::InProgress(127)),
                (true, InputState::InProgress(191)),
                (
                    false,
                    InputState::Active(Interaction::Navigation(Navigation::Next)),
                ),
            ],
            // long pulse recognised as Select event immediately
            &[
                (false, InputState::Idle),
                (true, InputState::Idle),
                (true, InputState::InProgress(63)),
                (true, InputState::InProgress(127)),
                (true, InputState::InProgress(191)),
                (
                    true,
                    InputState::Active(Interaction::Action(Action::Select)),
                ),
                (true, InputState::InProgress(51)),
                (true, InputState::InProgress(102)),
                (true, InputState::InProgress(153)),
                (true, InputState::InProgress(204)),
                (
                    true,
                    InputState::Active(Interaction::Action(Action::Select)),
                ),
                (true, InputState::InProgress(51)),
                (true, InputState::InProgress(102)),
                (true, InputState::InProgress(153)),
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
