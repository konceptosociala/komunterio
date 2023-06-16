use std::fs::{File, read_to_string};

use despero::prelude::*;

use crate::{
    ui::{
        set_gui_style, 
        menu::{
            ChapterSelectDialog, 
            SaveLoadDialog, 
            SettingsDialog
        }, 
        settings::Settings,
    }, 
    game::state::GameState,
};

pub struct SetupExtension;

impl Extension for SetupExtension {
    fn apply(&self, app: &mut Despero) {
        app
            .add_setup_system(main_setup)
            .add_system(set_gui_style);
    }
}

fn main_setup(
    mut cmd: Write<CommandBuffer>,
) -> DesperoResult<()> {    
    let settings = {
        use std::io::Write;

        let new_blank = || -> DesperoResult<Settings> {
            let mut file = File::create("config/settings.toml")?;
            let settings = Settings::default();
            
            let ser = toml::to_string_pretty(&settings).unwrap();
            file.write(ser.as_bytes())?;

            Ok(settings)
        };

        let settings = {
            if let Ok(config) = read_to_string("config/settings.toml") {
                match toml::from_str(&config) {
                    Ok(settings) => settings,
                    _ => new_blank()?,
                } 
            } else {
                new_blank()?
            }
        };

        settings
    };

    cmd.spawn((settings,));
    
    cmd.spawn((GameState::MainMenu(
        ChapterSelectDialog::default(),
        SaveLoadDialog::default(),
        SettingsDialog::default(),
    ),));

    Ok(())
}