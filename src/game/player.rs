use serde::{Serialize, Deserialize};
use flatbox::prelude::*;
use flatbox::audio_storage;

pub type UserID = u32;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Player(pub UserID);

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct PlayerCamera(pub UserID);

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Health(pub u32);

#[derive(Bundle, Debug)]
pub struct PlayerCameraBundle {
    pub player_camera: PlayerCamera,
    pub camera: Camera,
    pub internal_cast: AudioCast,
    pub listener: AudioListener,
    pub audio_storage: AudioStorage,
    pub transform: Transform,
}

impl PlayerCameraBundle {
    pub fn new(
        user_id: UserID,
        audio_manager: &mut AudioManager,
    ) -> Self {
        PlayerCameraBundle {
            player_camera: PlayerCamera(user_id),
            camera: Camera::builder()
                .camera_type(CameraType::FirstPerson)
                .is_active(true)
                .build(),
            internal_cast: AudioCast::new(audio_manager),
            listener: AudioListener::new(audio_manager),
            audio_storage: audio_storage![],
            transform: Transform::default(),
        }
    }
}

#[derive(Bundle, Debug)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub physics: BodyHandle,
    pub transform: Transform,
}

impl PlayerBundle {
    pub fn new(
        user_id: UserID,
        health: u32,
        physics_handler: &mut PhysicsHandler,
    ) -> Self {
        PlayerBundle {
            player: Player(user_id),
            health: Health(health),
            physics: physics_handler.new_instance(
                RigidBodyBuilder::dynamic().build(),
                ColliderBuilder::cuboid(0.5, 1.5, 0.5).build(),
            ),
            transform: Transform::default(),
        }
    }
}

pub fn spawn_player(
    mut cmd: Write<CommandBuffer>,
    mut physics_handler: Write<PhysicsHandler>,
    mut audio_manager: Write<AudioManager>,
){
    cmd.spawn(PlayerCameraBundle::new(1, &mut audio_manager));
    cmd.spawn(PlayerBundle::new(1, 100, &mut physics_handler));
}

pub fn process_player_camera(
    player_world: SubWorld<(&Transform, &Player)>,
    camera_world: SubWorld<(&mut Transform, &PlayerCamera)>,
){
    for (_, (t, ply_id)) in &mut player_world.query::<(&Transform, &Player)>(){
        for (_, (mut cam_t, cam_id)) in &mut camera_world.query::<(&mut Transform, &PlayerCamera)>(){
            if ply_id.0 == cam_id.0 {
                let mut new_t = t.clone();
                new_t.translation.y += 2.0;
                *cam_t = new_t;                
            }
        }
    }
}
