use serde::{Serialize, Deserialize};
use despero::prelude::*;

structstruck::strike! {
    #[strikethrough[derive(Default, Deserialize, Serialize, Debug)]]
    pub struct Settings {
        pub video: struct VideoSettings {
            pub resolution: enum Resolution {
                #[default]
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
            }
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