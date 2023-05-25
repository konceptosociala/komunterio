use despero::prelude::*;

use scene::manager::*;
use game::{
    state::*,
    player::*,
};
use ui::{
    set_gui_style,
    menu::*,
    splashscreen::*,
};

mod game;
mod ui;
mod scene;

fn main() {    
    Despero::init(
        WindowBuilder {
            title: Some("Komunterio"),
            fullscreen: Some(true),
            
            ..Default::default()
        }
    )
    
        .default_systems()
    
        .add_setup_system(main_setup)
        //~ .add_system(set_gui_style)
        //~ .add_system(splashscreen_loop)
        //~ .add_system(main_menu_system)
        
        //~ .add_system(scene_management)
        
        //~ .add_setup_system(add_timer)
        //~ .add_system(sleep_loop)
        
        .add_setup_system(spawn_player)
        .add_system(process_player_camera)
        
        .run();
}

fn main_setup(
    mut cmd: Write<CommandBuffer>,
    mut renderer: Write<Renderer>,
    mut physics_handler: Write<PhysicsHandler>
){    
    //~ cmd.spawn((GameState::MainMenu(false, false, false),));
    
    // Create a solid plane
    let texture = renderer.create_texture("assets/textures/dev/uv.jpg", Filter::LINEAR) as u32;
    
    cmd.spawn(
        ModelBundle::builder()
            .mesh(Mesh::load_obj("assets/models/skybox.obj").swap_remove(0))
            .material(
                renderer.create_material(
                    DefaultMat::builder()
                        .texture_id(texture)
                        .metallic(0.0)
                        .roughness(1.0)
                        .build()
                )
            )
            .transform(Transform::from_translation(Vector3::new(0.0, 0.0, 3.0)))
            .build()
    );
    
    let mut static_plane_builder = EntityBuilder::new();
    let static_plane = static_plane_builder
        .add_bundle(
            ModelBundle::builder()
                .mesh(Mesh::plane())
                .material(
                    renderer.create_material(
                        DefaultMat::builder()
                            .texture_id(texture)
                            .metallic(0.0)
                            .roughness(1.0)
                            .build()
                    )
                )
                .transform(Transform::new(
                    Vector3::new(0.0, 3.0, 0.0),
                    UnitQuaternion::from_axis_angle(&Vector3::x_axis(), to_radian(-90.0)),
                    1.0,
                ))
                .build()
        )
        .add(physics_handler.new_instance(
            RigidBodyBuilder::fixed().build(),
            ColliderBuilder::cuboid(1.0, 1.0, 0.1).build(),
        ))
        .build();
        
    cmd.spawn(static_plane);
}
