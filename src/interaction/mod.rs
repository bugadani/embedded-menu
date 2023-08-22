use core::hint::unreachable_unchecked;

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

impl InteractionType {
    /// Internal function to change the selection based on interaction. Moved
    /// separated to allow for easier testing.
    pub(crate) fn calculate_selection(self, selected: usize, count: usize) -> usize {
        // The lazy evaluation is necessary to prevent overflows.
        #[allow(clippy::unnecessary_lazy_evaluations)]
        match self {
            InteractionType::Next => (selected + 1) % count,
            InteractionType::Previous => selected.checked_sub(1).unwrap_or(count - 1),
            InteractionType::Select => unsafe {
                // This should be handled prior to calling this function. It is okay for
                // the compiler to optimize this away.
                unreachable_unchecked();
            },
            InteractionType::ForwardWrapping(n) => (selected + n) % count,
            InteractionType::Forward(n) => selected.saturating_add(n).min(count - 1),
            InteractionType::BackwardWrapping(n) => selected
                .checked_sub(n)
                .unwrap_or_else(|| count - (n - selected) % count),
            InteractionType::Backward(n) => selected.saturating_sub(n),
            InteractionType::Beginning => 0,
            InteractionType::End => count - 1,
            InteractionType::JumpTo(n) => n.min(count - 1),
        }
    }
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

#[test]
fn selection() {
    let count = 30;
    let mut selected = 3;
    for _ in 0..5 {
        selected = InteractionType::Previous.calculate_selection(selected, count);
    }
    assert_eq!(selected, 28);

    for _ in 0..5 {
        selected = InteractionType::Next.calculate_selection(selected, count);
    }
    assert_eq!(selected, 3);

    for _ in 0..5 {
        selected = InteractionType::BackwardWrapping(5).calculate_selection(selected, count);
    }
    assert_eq!(selected, 8);

    for _ in 0..5 {
        selected = InteractionType::ForwardWrapping(5).calculate_selection(selected, count);
    }
    assert_eq!(selected, 3);

    selected = InteractionType::JumpTo(20).calculate_selection(selected, count);
    assert_eq!(selected, 20);

    selected = InteractionType::Beginning.calculate_selection(selected, count);
    assert_eq!(selected, 0);

    selected = InteractionType::End.calculate_selection(selected, count);
    assert_eq!(selected, 29);

    for _ in 0..5 {
        selected = InteractionType::Backward(5).calculate_selection(selected, count);
    }
    assert_eq!(selected, 4);

    for _ in 0..5 {
        selected = InteractionType::Forward(5).calculate_selection(selected, count);
    }
    assert_eq!(selected, 29);
}

#[test]
fn selection_large_stupid_numbers() {
    let count = 30;
    let mut selected = 3;

    selected = InteractionType::BackwardWrapping(75).calculate_selection(selected, count);
    assert_eq!(selected, 18);

    selected = InteractionType::ForwardWrapping(75).calculate_selection(selected, count);
    assert_eq!(selected, 3);

    selected = InteractionType::BackwardWrapping(100000).calculate_selection(selected, count);
    assert_eq!(selected, 23);

    selected = InteractionType::ForwardWrapping(100000).calculate_selection(selected, count);
    assert_eq!(selected, 3);

    selected = InteractionType::JumpTo(100).calculate_selection(selected, count);
    assert_eq!(selected, 29);

    selected = InteractionType::JumpTo(0).calculate_selection(selected, count);
    assert_eq!(selected, 0);

    selected = InteractionType::Forward(100000).calculate_selection(selected, count);
    assert_eq!(selected, 29);

    selected = InteractionType::Backward(100000).calculate_selection(selected, count);
    assert_eq!(selected, 0);
}
