use despero::prelude::*;

use scene::manager::*;
use game::state::GameState;
use ui::{
    menu::*,
    splashscreen::*,
};

mod game;
mod ui;
mod scene;

fn main() {    
    Despero::init(
        WindowBuilder::new()
            .with_maximized(true)
            .with_title("Komunterio")
    )
    
        .add_setup_system(main_setup)
        .add_system(set_gui_style)
        .add_system(splashscreen_loop)
        .add_system(main_menu_system)
        
        .add_system(scene_management)
        
        .run();
}

fn main_setup(
    mut cmd: Write<CommandBuffer>,
){    
    cmd.spawn((GameState::MainMenu(false, false, false),));
    
    cmd.spawn(CameraBundle{
        camera: 
            Camera::builder()
                .is_active(true)
                .build(),
        transform: Transform::default(),
    });
}

fn set_gui_style(
    gui_ctx: Read<EventHandler<GuiContext>>,
){
    use gui::{
        TextStyle::*,
        FontFamily::Proportional,
        FontId,
    };
    
    if let Some(ctx) = gui_ctx.read() {
        let mut style = (*ctx.style()).clone();

        style.text_styles = [
          (Heading, FontId::new(27.5, Proportional)),
          (Name("Heading2".into()), FontId::new(25.0, Proportional)),
          (Name("Context".into()), FontId::new(23.0, Proportional)),
          (Body, FontId::new(18.0, Proportional)),
          (Monospace, FontId::new(14.0, Proportional)),
          (Button, FontId::new(14.0, Proportional)),
          (Small, FontId::new(10.0, Proportional)),
        ].into();
        
        let mut fonts = gui::FontDefinitions::default();
        
        fonts.font_data.insert(
            "GNUTypewriter".to_owned(), 
            gui::FontData::from_static(include_bytes!("../assets/fonts/gnu_typewriter/GNUTypewriter.ttf")),
        );
        
        fonts.families
            .get_mut(&gui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "GNUTypewriter".to_owned());
            
        ctx.set_style(style);
        ctx.set_fonts(fonts);
    }
}
