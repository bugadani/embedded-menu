pub mod programmed;
pub mod single_touch;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InteractionType {
    /// Equivalent to `BackwardWrapping(1)`, kept for backward compatibility.
    Previous,
    /// Equivalent to `ForwardWrapping(1)`, kept for backward compatibility.
    Next,
    /// Move the selection forward by `usize` items, wrapping around to the beginning if necessary.
    ForwardWrapping(usize),
    /// Move the selection forward by `usize` items, clamping at the end if necessary.
    Forward(usize),
    /// Move the selection backward by `usize` items, wrapping around to the end if necessary.
    BackwardWrapping(usize),
    /// Move the selection backward by `usize` items, clamping at the beginning if necessary.
    Backward(usize),
    /// Equivalent to `JumpTo(0)`, but simpler in semantics.
    Beginning,
    /// Equivalent to `JumpTo(usize::MAX)`, but simpler in semantics.
    End,
    /// Jump to the `usize`th item in the list, clamping at the beginning and end if necessary.
    JumpTo(usize),
    /// Select the currently selected item, executing any relevant action.
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
