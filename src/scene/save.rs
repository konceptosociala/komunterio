use serde::{Serialize, Deserialize};
use sonja::prelude::*;
use sonja::impl_save_load;

use crate::game::player::{Player, PlayerCamera, Health};

#[derive(Default, Deserialize, Serialize)]
pub struct SaveLoader {
    components: Vec<String>,
}

impl SaveLoader {
    pub fn new() -> Self {
        SaveLoader::default()
    }
}

impl_save_load!(
    loader: SaveLoader,
    components: [
        // Default components
        AssetHandle<'M'>,
        AssetHandle<'S'>,
        AssetHandle<'T'>,
        BodyHandle,
        Camera,
        DirectionalLight,
        Model,
        PointLight,
        Timer,
        Transform,
        // Player components
        Player,
        PlayerCamera,
        Health
    ]
);