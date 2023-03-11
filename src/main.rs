use despero::prelude::*;

use scene::manager::*;
use game::state::GameState;
use ui::{
    menu::*,
    splashscreen::*,
};

mod game;
mod ui;
mod scene;

fn main() {    
    Despero::init(
        WindowBuilder::new()
            .with_maximized(true)
            .with_title("Komunterio")
    )
    
        .add_setup_system(main_setup)
        .add_system(splashscreen_loop)
        .add_system(main_menu_system)
        
        .add_system(scene_management)
        
        .run();
}

fn main_setup(
    mut cmd: Write<CommandBuffer>,
){
    cmd.spawn((GameState::MainMenu(false, false),));
    
    cmd.spawn(CameraBundle{
        camera: 
            Camera::builder()
                .is_active(true)
                .build(),
        transform: Transform::default(),
    });
}
