pub mod programmed;
pub mod single_touch;

#[cfg(feature = "simulator")]
pub mod simulator;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interaction<R> {
    /// Change the selection
    Navigation(Navigation),
    /// Return a value
    Action(Action<R>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Action<R> {
    /// Select the currently selected item, executing any relevant action.
    Select,
    /// Return a value
    Return(R),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[must_use]
pub enum Navigation {
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
}

impl Navigation {
    /// Internal function to change the selection based on interaction.
    /// Separated to allow for easier testing.
    pub(crate) fn calculate_selection(
        self,
        mut selected: usize,
        count: usize,
        selectable: impl Fn(usize) -> bool,
    ) -> usize {
        // Clamp the selection to the range of selectable items.
        selected = selected.clamp(0, count - 1);
        let original = selected;

        // The lazy evaluation is necessary to prevent overflows.
        #[allow(clippy::unnecessary_lazy_evaluations)]
        match self {
            Self::Next => loop {
                selected = (selected + 1) % count;
                if selectable(selected) {
                    break selected;
                }
                // Prevent infinite loop if nothing is selectable.
                else if selected == original {
                    return 0;
                }
            },
            Self::Previous => loop {
                selected = selected.checked_sub(1).unwrap_or(count - 1);
                if selectable(selected) {
                    break selected;
                }
                // Prevent infinite loop if nothing is selectable.
                else if selected == original {
                    return 0;
                }
            },
            Self::ForwardWrapping(n) => {
                selected = (selected + n) % count;
                if !selectable(selected) {
                    Self::Next.calculate_selection(selected, count, selectable)
                } else {
                    selected
                }
            }
            Self::Forward(n) => {
                selected = selected.saturating_add(n).min(count - 1);
                if !selectable(selected) {
                    Self::Next.calculate_selection(selected, count, selectable)
                } else {
                    selected
                }
            }
            Self::BackwardWrapping(n) => {
                selected = selected
                    .checked_sub(n)
                    .unwrap_or_else(|| count - (n - selected) % count);
                if !selectable(selected) {
                    Self::Previous.calculate_selection(selected, count, selectable)
                } else {
                    selected
                }
            }
            Self::Backward(n) => {
                selected = selected.saturating_sub(n);
                if !selectable(selected) {
                    Self::Previous.calculate_selection(selected, count, selectable)
                } else {
                    selected
                }
            }
            Self::Beginning => {
                if !selectable(0) {
                    Self::Next.calculate_selection(0, count, selectable)
                } else {
                    0
                }
            }
            Self::End => {
                if !selectable(count - 1) {
                    Self::Previous.calculate_selection(count - 1, count, selectable)
                } else {
                    count - 1
                }
            }
            Self::JumpTo(n) => {
                selected = n.min(count - 1);
                if !selectable(selected) {
                    Self::Next.calculate_selection(selected, count, selectable)
                } else {
                    selected
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputResult<R> {
    StateUpdate(InputState),
    Interaction(Interaction<R>),
}

impl<R> From<Interaction<R>> for InputResult<R> {
    fn from(interaction: Interaction<R>) -> Self {
        Self::Interaction(interaction)
    }
}

impl<R> From<InputState> for InputResult<R> {
    fn from(state: InputState) -> Self {
        Self::StateUpdate(state)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InputState {
    Idle,
    InProgress(u8),
}

pub trait InputAdapterSource<R>: Copy {
    type InputAdapter: InputAdapter<Value = R>;

    fn adapter(&self) -> Self::InputAdapter;
}

pub trait InputAdapter: Copy {
    type Input;
    type Value;
    type State: Default + Copy;

    fn handle_input(
        &self,
        state: &mut Self::State,
        action: Self::Input,
    ) -> InputResult<Self::Value>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn selection() {
        let count = 30;
        let mut selected = 3;
        for _ in 0..5 {
            selected = Navigation::Previous.calculate_selection(selected, count, |_| true);
        }
        assert_eq!(selected, 28);

        for _ in 0..5 {
            selected = Navigation::Next.calculate_selection(selected, count, |_| true);
        }
        assert_eq!(selected, 3);

        for _ in 0..5 {
            selected =
                Navigation::BackwardWrapping(5).calculate_selection(selected, count, |_| true);
        }
        assert_eq!(selected, 8);

        for _ in 0..5 {
            selected =
                Navigation::ForwardWrapping(5).calculate_selection(selected, count, |_| true);
        }
        assert_eq!(selected, 3);

        selected = Navigation::JumpTo(20).calculate_selection(selected, count, |_| true);
        assert_eq!(selected, 20);

        selected = Navigation::Beginning.calculate_selection(selected, count, |_| true);
        assert_eq!(selected, 0);

        selected = Navigation::End.calculate_selection(selected, count, |_| true);
        assert_eq!(selected, 29);

        for _ in 0..5 {
            selected = Navigation::Backward(5).calculate_selection(selected, count, |_| true);
        }
        assert_eq!(selected, 4);

        for _ in 0..5 {
            selected = Navigation::Forward(5).calculate_selection(selected, count, |_| true);
        }
        assert_eq!(selected, 29);
    }

    #[test]
    fn selection_large_stupid_numbers() {
        let count = 30;
        let mut selected = 3;

        selected = Navigation::BackwardWrapping(75).calculate_selection(selected, count, |_| true);
        assert_eq!(selected, 18);

        selected = Navigation::ForwardWrapping(75).calculate_selection(selected, count, |_| true);
        assert_eq!(selected, 3);

        selected =
            Navigation::BackwardWrapping(100000).calculate_selection(selected, count, |_| true);
        assert_eq!(selected, 23);

        selected =
            Navigation::ForwardWrapping(100000).calculate_selection(selected, count, |_| true);
        assert_eq!(selected, 3);

        selected = Navigation::JumpTo(100).calculate_selection(selected, count, |_| true);
        assert_eq!(selected, 29);

        selected = Navigation::JumpTo(0).calculate_selection(selected, count, |_| true);
        assert_eq!(selected, 0);

        selected = Navigation::Forward(100000).calculate_selection(selected, count, |_| true);
        assert_eq!(selected, 29);

        selected = Navigation::Backward(100000).calculate_selection(selected, count, |_| true);
        assert_eq!(selected, 0);
    }

    #[test]
    fn unselectable_selection_infinite_loop() {
        let selected = Navigation::BackwardWrapping(75).calculate_selection(5, 10, |_| false);
        assert_eq!(selected, 0);
        let selected = Navigation::ForwardWrapping(75).calculate_selection(5, 10, |_| false);
        assert_eq!(selected, 0);
        let selected = Navigation::BackwardWrapping(75).calculate_selection(5, 10, |_| false);
        assert_eq!(selected, 0);
        let selected = Navigation::ForwardWrapping(75).calculate_selection(5, 10, |_| false);
        assert_eq!(selected, 0);
        let selected = Navigation::JumpTo(75).calculate_selection(5, 10, |_| false);
        assert_eq!(selected, 0);
        let selected = Navigation::JumpTo(75).calculate_selection(5, 10, |_| false);
        assert_eq!(selected, 0);
        let selected = Navigation::Forward(75).calculate_selection(5, 10, |_| false);
        assert_eq!(selected, 0);
        let selected = Navigation::Backward(75).calculate_selection(5, 10, |_| false);
        assert_eq!(selected, 0);
        let selected = Navigation::Next.calculate_selection(5, 10, |_| false);
        assert_eq!(selected, 0);
        let selected = Navigation::Previous.calculate_selection(5, 10, |_| false);
        assert_eq!(selected, 0);
        let selected = Navigation::Beginning.calculate_selection(5, 10, |_| false);
        assert_eq!(selected, 0);
        let selected = Navigation::End.calculate_selection(5, 10, |_| false);
        assert_eq!(selected, 0);
    }
}
