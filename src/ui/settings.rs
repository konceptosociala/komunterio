use std::fmt::Display;
use std::path::Path;
use std::fs::{File, read_to_string};
use std::io::Write;

use serde::{Serialize, Deserialize};
use flatbox::prelude::*;

pub struct ApplySettings;

structstruck::strike! {
    #[strikethrough[derive(Default, Deserialize, Serialize, Debug, PartialEq, Eq)]]
    pub struct Settings {
        pub video: struct VideoSettings {
            pub resolution: enum Resolution {
                #[default]
                System,
                R1920x1080,
                R1280x720,
                R1024x576,
                R640x360,
            },
            pub gui_scale: enum GuiScale {
                Tiny,
                #[default]
                Normal,
                Large,
            },
            pub fullscreen: bool,
        },
        pub sound: struct SoundSettings {
            pub music_volume: Volume,
            pub sound_volume: Volume,
            pub voice_volume: Volume,
        },
        pub game: struct GameSettings {
            // ...
        },
        pub controls: struct ControlSettings {
            // ...
        },
    }
}

impl Settings {
    pub fn load(path: impl AsRef<Path> + Copy) -> FlatboxResult<Self> {
        let settings = {
            if let Ok(config) = read_to_string(path) {
                match toml::from_str(&config) {
                    Ok(settings) => settings,
                    _ => Self::new_blank(path)?,
                } 
            } else {
                Self::new_blank(path)?
            }
        };

        Ok(settings)
    }

    pub fn new_blank(path: impl AsRef<Path>) -> FlatboxResult<Self> {
        let mut file = File::create(path)?;
        let settings = Settings::default();
        
        let ser = toml::to_string_pretty(&settings).unwrap();
        file.write(ser.as_bytes())?;

        Ok(settings)
    }
}

impl Display for Resolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Resolution::System => "System",
            Resolution::R1920x1080 => "1920x1080",
            Resolution::R1280x720 => "1280x720",
            Resolution::R1024x576 => "1024x576",
            Resolution::R640x360 => "640x360",
        })
    }
}

impl Display for GuiScale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GuiScale::Tiny => "Tiny",
            GuiScale::Normal => "Normal",
            GuiScale::Large => "Large",
        })
    }
}