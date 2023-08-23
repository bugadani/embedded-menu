use core::hint::unreachable_unchecked;

pub mod programmed;
pub mod single_touch;

#[cfg(feature = "simulator")]
pub mod simulator;

#[derive(Clone, Copy)]
pub enum NoAction {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interaction<R> {
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
    /// Return a value
    Action(R),
}

impl<R> Interaction<R> {
    /// Internal function to change the selection based on interaction.
    /// Separated to allow for easier testing.
    pub(crate) fn calculate_selection(self, selected: usize, count: usize) -> usize {
        // The lazy evaluation is necessary to prevent overflows.
        #[allow(clippy::unnecessary_lazy_evaluations)]
        match self {
            Interaction::Next => (selected + 1) % count,
            Interaction::Previous => selected.checked_sub(1).unwrap_or(count - 1),
            Interaction::ForwardWrapping(n) => (selected + n) % count,
            Interaction::Forward(n) => selected.saturating_add(n).min(count - 1),
            Interaction::BackwardWrapping(n) => selected
                .checked_sub(n)
                .unwrap_or_else(|| count - (n - selected) % count),
            Interaction::Backward(n) => selected.saturating_sub(n),
            Interaction::Beginning => 0,
            Interaction::End => count - 1,
            Interaction::JumpTo(n) => n.min(count - 1),
            Interaction::Select | Interaction::Action(_) => unsafe {
                // This should be handled prior to calling this function. It is okay for
                // the compiler to optimize this away.
                unreachable_unchecked()
            },
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputState<R> {
    Idle,
    InProgress(u8),
    Active(Interaction<R>),
}

pub trait InputAdapterSource<R>: Copy {
    type InputAdapter: InputAdapter<Value = R>;

    fn adapter(&self) -> Self::InputAdapter;
}

pub trait InputAdapter: Copy {
    type Input;
    type Value: Copy;
    type State: Default + Copy;

    fn handle_input(&self, state: &mut Self::State, action: Self::Input)
        -> InputState<Self::Value>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn selection() {
        let count = 30;
        let mut selected = 3;
        for _ in 0..5 {
            selected = Interaction::<NoAction>::Previous.calculate_selection(selected, count);
        }
        assert_eq!(selected, 28);

        for _ in 0..5 {
            selected = Interaction::<NoAction>::Next.calculate_selection(selected, count);
        }
        assert_eq!(selected, 3);

        for _ in 0..5 {
            selected =
                Interaction::<NoAction>::BackwardWrapping(5).calculate_selection(selected, count);
        }
        assert_eq!(selected, 8);

        for _ in 0..5 {
            selected =
                Interaction::<NoAction>::ForwardWrapping(5).calculate_selection(selected, count);
        }
        assert_eq!(selected, 3);

        selected = Interaction::<NoAction>::JumpTo(20).calculate_selection(selected, count);
        assert_eq!(selected, 20);

        selected = Interaction::<NoAction>::Beginning.calculate_selection(selected, count);
        assert_eq!(selected, 0);

        selected = Interaction::<NoAction>::End.calculate_selection(selected, count);
        assert_eq!(selected, 29);

        for _ in 0..5 {
            selected = Interaction::<NoAction>::Backward(5).calculate_selection(selected, count);
        }
        assert_eq!(selected, 4);

        for _ in 0..5 {
            selected = Interaction::<NoAction>::Forward(5).calculate_selection(selected, count);
        }
        assert_eq!(selected, 29);
    }

    #[test]
    fn selection_large_stupid_numbers() {
        let count = 30;
        let mut selected = 3;

        selected =
            Interaction::<NoAction>::BackwardWrapping(75).calculate_selection(selected, count);
        assert_eq!(selected, 18);

        selected =
            Interaction::<NoAction>::ForwardWrapping(75).calculate_selection(selected, count);
        assert_eq!(selected, 3);

        selected =
            Interaction::<NoAction>::BackwardWrapping(100000).calculate_selection(selected, count);
        assert_eq!(selected, 23);

        selected =
            Interaction::<NoAction>::ForwardWrapping(100000).calculate_selection(selected, count);
        assert_eq!(selected, 3);

        selected = Interaction::<NoAction>::JumpTo(100).calculate_selection(selected, count);
        assert_eq!(selected, 29);

        selected = Interaction::<NoAction>::JumpTo(0).calculate_selection(selected, count);
        assert_eq!(selected, 0);

        selected = Interaction::<NoAction>::Forward(100000).calculate_selection(selected, count);
        assert_eq!(selected, 29);

        selected = Interaction::<NoAction>::Backward(100000).calculate_selection(selected, count);
        assert_eq!(selected, 0);
    }
}
