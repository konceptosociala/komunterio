use despero::prelude::*;

use crate::game::state::GameState;

pub fn splashscreen_loop(
    state_world: SubWorld<&GameState>,
){
    let mut binding = state_world.query::<&GameState>();
    let (_, state) = binding.iter().next().unwrap();
    
    match state {
        GameState::Splashscreen => {},
        _ => return,
    }
    
    println!("Splashscreen");
}
