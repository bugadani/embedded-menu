use crate::interaction::{InteractionController, InteractionType};

pub struct Programmed;

impl InteractionController for Programmed {
    type Input = InteractionType;

    fn reset(&mut self) {}
    fn fill_area_width(&self, _max: u32) -> u32 {
        0
    }
    fn update(&mut self, action: Self::Input) -> Option<InteractionType> {
        Some(action)
    }
}
