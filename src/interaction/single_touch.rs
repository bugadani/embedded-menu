use core::marker::PhantomData;

use crate::{
    interaction::{
        Action, InputAdapter, InputAdapterSource, InputResult, InputState, Interaction, Navigation,
    },
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

impl<R> InputAdapterSource<R> for SingleTouch {
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
pub struct SingleTouchAdapter<R> {
    ignore_time: u32,
    debounce_time: u32,
    max_time: u32,
    marker: PhantomData<R>,
}

impl<R> Clone for SingleTouchAdapter<R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<R> Copy for SingleTouchAdapter<R> {}

impl<R> InputAdapter for SingleTouchAdapter<R> {
    type Input = bool;
    type Value = R;
    type State = State;

    fn handle_input(
        &self,
        state: &mut Self::State,
        action: Self::Input,
    ) -> InputResult<Self::Value> {
        if !state.was_released {
            if action {
                return InputResult::from(InputState::Idle);
            }
            state.was_released = true;
        }

        if action {
            state.interaction_time = state.interaction_time.saturating_add(1);
            if state.interaction_time <= self.ignore_time && !state.repeated {
                InputResult::from(InputState::Idle)
            } else if state.interaction_time < self.max_time {
                let ignore_time = if state.repeated { 0 } else { self.ignore_time };
                InputResult::from(InputState::InProgress(interpolate(
                    state.interaction_time - ignore_time,
                    0,
                    self.max_time - ignore_time,
                    0,
                    255,
                ) as u8))
            } else {
                state.repeated = true;
                state.interaction_time = 0;
                InputResult::from(Interaction::Action(Action::Select))
            }
        } else {
            let time = core::mem::replace(&mut state.interaction_time, 0);

            if self.debounce_time < time && time < self.max_time && !state.repeated {
                InputResult::from(Interaction::Navigation(Navigation::Next))
            } else {
                // Already interacted before releasing, ignore and reset.
                state.repeated = false;
                InputResult::from(InputState::Idle)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::interaction::{
        single_touch::SingleTouch, Action, InputAdapter, InputAdapterSource, InputResult,
        InputState, Interaction, Navigation,
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

        let expectations: [&[(bool, InputResult<()>)]; 6] = [
            &[
                (false, InputState::Idle.into()),
                (false, InputState::Idle.into()),
                (false, InputState::Idle.into()),
                (false, InputState::Idle.into()),
                (false, InputState::Idle.into()),
                (false, InputState::Idle.into()),
            ],
            // repeated short pulse ignored
            &[
                (true, InputState::Idle.into()),
                (false, InputState::Idle.into()),
                (true, InputState::Idle.into()),
                (false, InputState::Idle.into()),
                (true, InputState::Idle.into()),
                (false, InputState::Idle.into()),
            ],
            // longer pulse recongised as Next event on falling edge
            &[
                (false, InputState::Idle.into()),
                (true, InputState::Idle.into()),
                (true, InputState::InProgress(63).into()),
                (false, Interaction::Navigation(Navigation::Next).into()),
            ],
            &[
                (false, InputState::Idle.into()),
                (true, InputState::Idle.into()),
                (true, InputState::InProgress(63).into()),
                (true, InputState::InProgress(127).into()),
                (false, Interaction::Navigation(Navigation::Next).into()),
            ],
            // long pulse NOT recognised as Select event on falling edge
            &[
                (false, InputState::Idle.into()),
                (true, InputState::Idle.into()),
                (true, InputState::InProgress(63).into()),
                (true, InputState::InProgress(127).into()),
                (true, InputState::InProgress(191).into()),
                (false, Interaction::Navigation(Navigation::Next).into()),
            ],
            // long pulse recognised as Select event immediately
            &[
                (false, InputState::Idle.into()),
                (true, InputState::Idle.into()),
                (true, InputState::InProgress(63).into()),
                (true, InputState::InProgress(127).into()),
                (true, InputState::InProgress(191).into()),
                (true, Interaction::Action(Action::Select).into()),
                (true, InputState::InProgress(51).into()),
                (true, InputState::InProgress(102).into()),
                (true, InputState::InProgress(153).into()),
                (true, InputState::InProgress(204).into()),
                (true, Interaction::Action(Action::Select).into()),
                (true, InputState::InProgress(51).into()),
                (true, InputState::InProgress(102).into()),
                (true, InputState::InProgress(153).into()),
                (false, InputState::Idle.into()),
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
