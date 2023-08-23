use core::marker::PhantomData;

use crate::interaction::{InputAdapter, InputAdapterSource, InputState, Interaction};

#[derive(Clone, Copy)]
pub struct Programmed;

impl<R> InputAdapterSource<R> for Programmed
where
    R: Copy,
{
    type InputAdapter = ProgrammedAdapter<R>;

    fn adapter(&self) -> Self::InputAdapter {
        ProgrammedAdapter {
            _marker: PhantomData,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ProgrammedAdapter<R>
where
    R: Copy,
{
    _marker: PhantomData<R>,
}

impl<R> InputAdapter for ProgrammedAdapter<R>
where
    R: Copy,
{
    type Input = Interaction<R>;
    type Value = R;
    type State = ();

    fn handle_input(
        &self,
        _state: &mut Self::State,
        action: Self::Input,
    ) -> InputState<Self::Value> {
        InputState::Active(action)
    }
}
