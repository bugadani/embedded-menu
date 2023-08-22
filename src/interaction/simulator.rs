use embedded_graphics_simulator::{sdl2::Keycode, SimulatorEvent};

use crate::interaction::{InputAdapter, InputState, InteractionType};

/// Input adapter to work with the embedded-graphics simulator
#[derive(Clone, Copy)]
pub struct Simulator {
    /// Number of menu items to skip when pressing page up or page down.
    pub page_size: usize,
}

impl Default for Simulator {
    fn default() -> Self {
        Self { page_size: 5 }
    }
}

impl InputAdapter for Simulator {
    type Input = SimulatorEvent;
    type State = ();

    fn handle_input(&self, _state: &mut Self::State, action: Self::Input) -> InputState {
        match action {
            SimulatorEvent::KeyDown { repeat: false, .. } => return InputState::Idle,
            SimulatorEvent::KeyDown { repeat: true, .. } => return InputState::InProgress(255),
            SimulatorEvent::KeyUp { keycode, .. } => match keycode {
                Keycode::Return => InputState::Active(InteractionType::Select),
                Keycode::Up => InputState::Active(InteractionType::Previous),
                Keycode::Down => InputState::Active(InteractionType::Next),
                Keycode::PageDown => InputState::Active(InteractionType::Forward(self.page_size)),
                Keycode::PageUp => InputState::Active(InteractionType::Backward(self.page_size)),
                _ => InputState::Idle,
            },
            _ => InputState::Idle,
        }
    }
}
