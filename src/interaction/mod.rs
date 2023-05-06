pub mod programmed;
pub mod single_touch;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InteractionType {
    Previous,
    Next,
    Select,
}

pub trait InteractionController: Copy {
    type Input;
    type State: Default;

    fn reset(&self, _state: &mut Self::State) {}
    fn fill_area_width(&self, state: &Self::State, max: u32) -> u32;
    fn update(&self, state: &mut Self::State, action: Self::Input) -> Option<InteractionType>;
}
