#[derive(Debug)]
pub enum GameState {    
    Splashscreen,
    MainMenu(bool, bool),
    
    Loading,
    Game,
    Pause,
}
