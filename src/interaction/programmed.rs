use core::marker::PhantomData;

use crate::interaction::{InputAdapter, InputAdapterSource, InputResult, Interaction};

#[derive(Clone, Copy)]
pub struct Programmed;

impl<R> InputAdapterSource<R> for Programmed {
    type InputAdapter = ProgrammedAdapter<R>;

    fn adapter(&self) -> Self::InputAdapter {
        ProgrammedAdapter {
            _marker: PhantomData,
        }
    }
}

pub struct ProgrammedAdapter<R> {
    _marker: PhantomData<R>,
}

impl<R> Clone for ProgrammedAdapter<R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<R> Copy for ProgrammedAdapter<R> {}

impl<R> InputAdapter for ProgrammedAdapter<R> {
    type Input = Interaction<R>;
    type Value = R;
    type State = ();

    fn handle_input(
        &self,
        _state: &mut Self::State,
        action: Self::Input,
    ) -> InputResult<Self::Value> {
        InputResult::from(action)
    }
}
