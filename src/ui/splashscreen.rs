use std::time::Duration;
use despero::prelude::*;

use crate::game::state::GameState;

pub struct SplashTimer {
    timer: Timer,
}

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

fn add_timer(
    mut cmd: Write<CommandBuffer>,
){
    cmd.spawn((SplashTimer {
        timer: Timer::init(Duration::from_secs(5), false),
    },));
}

fn sleep_loop(
    timer_world: SubWorld<&mut SplashTimer>,
    time: Read<Time>,
){
    for (_, mut st) in &mut timer_world.query::<&mut SplashTimer>() {
        st.timer.tick(time.delta_time());
        
        if st.timer.finished() {
            info!("Timer finished!");
        }
    }
}
