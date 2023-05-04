pub mod programmed;
pub mod single_touch;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InteractionType {
    Nothing,
    Previous,
    Next,
    Select,
}

pub trait InputType: Copy + Sized {
    fn is_active(self) -> bool;
}

impl InputType for bool {
    fn is_active(self) -> bool {
        self
    }
}

impl InputType for InteractionType {
    fn is_active(self) -> bool {
        self != InteractionType::Nothing
    }
}

pub trait InteractionController {
    type Input: core::fmt::Debug + InputType;

    fn reset(&mut self) {}
    fn fill_area_width(&self, max: u32) -> u32;
    fn update(&mut self, action: Self::Input) -> InteractionType;
}
