pub mod programmed;
pub mod single_touch;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InteractionType {
    Previous,
    Next,
    Select,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputState {
    Idle,
    InProgress(u8),
    Active(InteractionType),
}

pub trait InputAdapter: Copy {
    type Input;
    type State: Default + Copy;

    fn handle_input(&self, state: &mut Self::State, action: Self::Input) -> InputState;
}
