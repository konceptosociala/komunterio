mod game;
mod ui;
mod scene;
mod setup;

use despero::prelude::*;

use setup::*;
use ui::menu::MainMenuExtension;

fn main() {    
    Despero::init(
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
