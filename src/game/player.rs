use despero::prelude::*;

pub type UserID = u32;
pub struct Player(UserID);
pub struct PlayerCamera(UserID);

pub struct Health(pub u32);

pub fn spawn_player(
    mut cmd: Write<CommandBuffer>,
    mut physics_handler: Write<PhysicsHandler>,
){
    // Summon player's camera
    cmd.spawn((
        Camera::builder()
            .is_active(true)
            .build(),
        Transform::default(),
        PlayerCamera(1),
    ));
    
    // Summon player
    cmd.spawn((
        physics_handler.new_instance(
            RigidBodyBuilder::dynamic().build(),
            ColliderBuilder::cuboid(0.5, 1.5, 0.5).build(),
        ),
        Transform::default(),
        Health(100),
        Player(1),
    ));
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
