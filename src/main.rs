mod game;
mod ui;
mod scene;
mod setup;

use sonja::prelude::*;

use setup::*;
use ui::menu::MainMenuExtension;

fn main() {    
    Sonja::init(
        WindowBuilder {
            title: Some("Komunterio"),
            fullscreen: Some(true),
            
            ..Default::default()
        }
    )
    
        .default_systems()
        
        .apply_extension(SetupExtension)
        .apply_extension(MainMenuExtension)
        
        .run();
}
