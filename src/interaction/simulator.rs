use embedded_graphics_simulator::{sdl2::Keycode, SimulatorEvent};

use crate::interaction::{InputAdapter, InputAdapterSource, InputState, InteractionType};

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
    ) -> InputState<Self::Value> {
        match action {
            SimulatorEvent::KeyDown { repeat: false, .. } => return InputState::Idle,
            SimulatorEvent::KeyDown { repeat: true, .. } => return InputState::InProgress(255),
            SimulatorEvent::KeyUp { keycode, .. } => match keycode {
                Keycode::Return => InputState::Active(InteractionType::Select),
                Keycode::Up => InputState::Active(InteractionType::Previous),
                Keycode::Down => InputState::Active(InteractionType::Next),
                Keycode::PageDown => InputState::Active(InteractionType::Forward(self.page_size)),
                Keycode::PageUp => InputState::Active(InteractionType::Backward(self.page_size)),
                Keycode::Escape => InputState::Active(InteractionType::Action(self.esc_value)),
                _ => InputState::Idle,
            },
            SimulatorEvent::Quit => InputState::Active(InteractionType::Action(self.esc_value)),
            _ => InputState::Idle,
        }
    }
}
