mod game;
mod ui;

use despero::prelude::*;

use game::state::GameState;
use ui::{
    menu::*,
    splashscreen::*,
};

fn main() {    
    let despero = Despero::init(
        WindowBuilder::new()
            .with_maximized(true)
            .with_title("Komunterio")
    );
        
    despero        
        .add_setup_system(main_setup)
        
        .add_system(splashscreen_loop)
        
        .add_system(main_menu_system)
        
        .run();
}

fn main_setup(
    mut cmd: Write<CommandBuffer>,
){
    cmd.spawn((GameState::MainMenu,));
    
    cmd.spawn(CameraBundle{
        camera: 
            Camera::builder()
                .is_active(true)
                .build(),
        transform: Transform::default(),
    });
}
