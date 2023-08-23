use embedded_graphics_simulator::{sdl2::Keycode, SimulatorEvent};

use crate::interaction::{
    Action, InputAdapter, InputAdapterSource, InputResult, InputState, Interaction, Navigation,
};

/// Input adapter to work with the embedded-graphics simulator
#[derive(Clone, Copy)]
pub struct Simulator<R>
where
    R: Copy,
{
    /// Number of menu items to skip when pressing page up or page down.
    pub page_size: usize,
    pub esc_value: R,
}

impl<R> InputAdapterSource<R> for Simulator<R>
where
    R: Copy,
{
    type InputAdapter = Self;

    fn adapter(&self) -> Self::InputAdapter {
        *self
    }
}

impl<R> InputAdapter for Simulator<R>
where
    R: Copy,
{
    type Input = SimulatorEvent;
    type Value = R;
    type State = ();

    fn handle_input(
        &self,
        _state: &mut Self::State,
        action: Self::Input,
    ) -> InputResult<Self::Value> {
        match action {
            SimulatorEvent::KeyDown { repeat: false, .. } => InputResult::from(InputState::Idle),
            SimulatorEvent::KeyDown { repeat: true, .. } => {
                InputResult::from(InputState::InProgress(255))
            }
            SimulatorEvent::KeyUp { keycode, .. } => match keycode {
                Keycode::Return => InputResult::from(Interaction::Action(Action::Select)),
                Keycode::Up => InputResult::from(Interaction::Navigation(Navigation::Previous)),
                Keycode::Down => InputResult::from(Interaction::Navigation(Navigation::Next)),
                Keycode::PageDown => {
                    InputResult::from(Interaction::Navigation(Navigation::Forward(self.page_size)))
                }
                Keycode::PageUp => InputResult::from(Interaction::Navigation(
                    Navigation::Backward(self.page_size),
                )),
                Keycode::Escape => {
                    InputResult::from(Interaction::Action(Action::Return(self.esc_value)))
                }
                _ => InputResult::from(InputState::Idle),
            },
            SimulatorEvent::Quit => {
                InputResult::from(Interaction::Action(Action::Return(self.esc_value)))
            }
            _ => InputResult::from(InputState::Idle),
        }
    }
}
