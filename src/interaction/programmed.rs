use crate::interaction::{InputAdapter, InputState, InteractionType};

#[derive(Clone, Copy)]
pub struct Programmed;

impl InputAdapter for Programmed {
    type Input = InteractionType;
    type State = ();

    fn handle_input(&self, _state: &mut Self::State, action: Self::Input) -> InputState {
        InputState::Active(action)
    }
}
