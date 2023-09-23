use std::marker::PhantomData;

use serde::{Serialize, Deserialize};
use serde_closure::{traits::Fn, Fn};
use flatbox::prelude::*;

use super::state::GameState;

#[derive(Serialize, Deserialize)]
pub struct Script<F, Args>
where
    F: Fn<Args>
{
    executor: F,
    _marker: PhantomData<Args>
}

impl<F, Args> Script<F, Args>
where
    F: Fn<Args>
{
    pub fn new(executor: F) -> Self {
        Script { 
            executor,
            _marker: PhantomData,
        }
    }
}

pub fn processing_level(
    state_world: SubWorld<&GameState>,
){
    let mut binding = state_world.query::<&GameState>();
    let (_, state) = binding.iter().next().unwrap();
    
    match state {
        GameState::Game => {},
        _ => return,
    }

    
}