use crate::interaction::{InteractionController, InteractionType};

#[derive(Clone, Copy)]
pub struct Programmed;

impl InteractionController for Programmed {
    type Input = InteractionType;
    type State = ();

    fn fill_area_width(&self, _state: &Self::State, _max: u32) -> u32 {
        0
    }

    fn update(&mut self, _state: &mut Self::State, action: Self::Input) -> Option<InteractionType> {
        Some(action)
    }
}
