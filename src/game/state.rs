#[derive(Debug)]
pub enum GameState {    
    Splashscreen,
    MainMenu(bool, bool, bool),
    
    Loading,
    Game,
    Pause,
}
