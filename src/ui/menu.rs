use despero::prelude::*;

use crate::game::state::*;

pub fn main_menu_system(
    state_world: SubWorld<&GameState>,
    egui_ctx: Write<EventHandler<GuiContext>>,
){
    let mut binding = state_world.query::<&GameState>();
    let (_, state) = binding.iter().next().unwrap();
    
    match state {
        GameState::MainMenu => {},
        _ => return,
    }
    
    if let Some(ctx) = egui_ctx.read() {
        
        egui::SidePanel::left("my_panel").show(&ctx, |ui| {
            
            ui.label("Komunterio");
            
            if ui.button("Ludi").clicked() {
                
            }
            
            if ui.button("Agordoj").clicked() {
                
            }
            
            if ui.button("Eliri").clicked() {
                
            }
            
        });
        
    }
}
