pub mod programmed;
pub mod single_touch;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InteractionType {
    Previous,
    Next,
    Select,
}

pub trait InteractionController {
    type Input;

    fn reset(&mut self) {}
    fn fill_area_width(&self, max: u32) -> u32;
    fn update(&mut self, action: Self::Input) -> Option<InteractionType>;
}
