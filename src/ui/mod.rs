use despero::prelude::*;

pub mod menu;
pub mod settings;
pub mod splashscreen;

pub fn set_gui_style(
    events: Read<Events>,
){
    use gui::{
        TextStyle::*,
        FontFamily::Proportional,
        FontId,
    };
    
    let gui_ctx = events.get_handler::<GuiContext>().unwrap();
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
            gui::FontData::from_static(include_bytes!("../../assets/fonts/gnu_typewriter/GNUTypewriter.ttf")),
        );
        
        fonts.families
            .get_mut(&gui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "GNUTypewriter".to_owned());
            
        ctx.set_style(style);
        ctx.set_fonts(fonts);
    }
}
