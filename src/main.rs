mod game;
mod ui;
mod scene;
mod setup;

use flatbox::prelude::*;

use setup::*;
use ui::menu::MainMenuExtension;

fn main() {    
    Flatbox::init(
        WindowBuilder {
            title: "Komunterio",
            fullscreen: true,
            
            ..Default::default()
        }
    )
    
        .default_systems()
        
        .apply_extension(SetupExtension)
        .apply_extension(MainMenuExtension)
        
        .run();
}
