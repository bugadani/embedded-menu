pub mod programmed;
pub mod single_touch;

/// Clamp the selection around the first and last item, preventing
/// wrapping to the beginning.
#[derive(Copy, Clone)]
pub struct Clamped<T: InteractionController>(pub T);

impl<T: InteractionController + Copy> InteractionController for Clamped<T> {
    type Input = T::Input;
    type State = T::State;

    fn fill_area_width(&self, state: &Self::State, max: u32) -> u32 {
        self.0.fill_area_width(state, max)
    }

    fn select(
        &self,
        _state: &mut Self::State,
        selected: usize,
        count: usize,
        interaction: InteractionType,
    ) -> usize {
        match interaction {
            InteractionType::Previous => {
                if selected == 0 {
                    0
                } else {
                    selected - 1
                }
            }
            InteractionType::Next => {
                if selected == count - 1 {
                    count - 1
                } else {
                    selected + 1
                }
            }
            InteractionType::Forward(i) => {
                if selected + i >= count {
                    count - 1
                } else {
                    selected + i
                }
            }
            InteractionType::Backward(i) => {
                if selected < i {
                    0
                } else {
                    selected - i
                }
            }
            InteractionType::Beginning => 0,
            InteractionType::End => count - 1,
            InteractionType::Select => selected,
        }
    }

    fn update(&self, state: &mut Self::State, action: Self::Input) -> Option<InteractionType> {
        self.0.update(state, action)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InteractionType {
    Previous,
    Next,
    Select,
    Forward(usize),
    Backward(usize),
    Beginning,
    End,
}

pub trait InteractionController: Copy {
    type Input;
    type State: Default + Copy;

    fn fill_area_width(&self, state: &Self::State, max: u32) -> u32;

    /// Update the selection index based on the interaction type returned by the
    /// `update()` function. By default, loops through the list normally.
    fn select(
        &self,
        _state: &mut Self::State,
        selected: usize,
        count: usize,
        interaction: InteractionType,
    ) -> usize {
        match interaction {
            InteractionType::Previous => selected.checked_sub(1).unwrap_or(count - 1),
            InteractionType::Next => (selected + 1) % count,
            InteractionType::Forward(i) => {
                if selected == count - 1 {
                    0
                } else if selected + i >= count {
                    count - 1
                } else {
                    selected + i
                }
            }
            InteractionType::Backward(i) => {
                if selected == 0 {
                    count - 1
                } else if selected < i {
                    0
                } else {
                    selected - i
                }
            }
            InteractionType::Beginning => 0,
            InteractionType::End => count - 1,
            InteractionType::Select => selected,
        }
    }

    /// Transforms an input into an interaction type, applying logic on the
    /// state if needed.
    fn update(&self, state: &mut Self::State, action: Self::Input) -> Option<InteractionType>;
}
