use crate::ui::menu::{ChapterSelectDialog, SaveLoadDialog, SettingsDialog};

#[derive(Debug)]
pub enum GameState {    
    Splashscreen,

    MainMenu(
        ChapterSelectDialog, 
        SaveLoadDialog, 
        SettingsDialog
    ),
    
    Loading,
    Game,
    Pause,
}
