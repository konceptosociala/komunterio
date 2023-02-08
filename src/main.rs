use despero::prelude::*;

fn main() {    
    let despero = Despero::init(
        WindowBuilder::new()
            .with_maximized(true)
            .with_title("Komunterio")
    );
        
    despero
        .add_setup_system(create_model)
        .add_setup_system(create_camera)
        .add_system(rotate_model)
        .run();
}

fn create_model(
    mut cmd: Write<CommandBuffer>,
    mut renderer: Write<Renderer>,
){
    let texture = renderer.create_texture("assets/uv.jpg", Filter::LINEAR) as u32;
    
    cmd.spawn(ModelBundle {
        mesh: Mesh::load_obj("assets/model.obj").swap_remove(0),
        material: renderer.create_material(
            DefaultMat::builder()
                .texture_id(texture)
                .metallic(0.0)
                .roughness(1.0)
                .build(),
        ),
        transform: Transform::from_translation(Vector3::new(0.0, 0.0, 0.0)),
    });       
}

fn rotate_model(
    world: SubWorld<&mut Transform>,
){
    for (_, mut t) in &mut world.query::<&mut Transform>() {
        t.rotation *= UnitQuaternion::from_axis_angle(&Unit::new_normalize(Vector3::new(0.0, 1.0, 0.0)), 0.05);
    }
} 

fn create_camera(
    mut cmd: Write<CommandBuffer>,
){
    cmd.spawn(CameraBundle{
        camera: 
            Camera::builder()
                .is_active(true)
                .build(),
        transform: Transform::default(),
    });
}
