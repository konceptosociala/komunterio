use despero::prelude::*;

use crate::game::state::*;

pub fn main_menu_system(
    state_world: SubWorld<&mut GameState>,
    egui_ctx: Read<EventHandler<GuiContext>>,
    mut app_exit: Write<EventHandler<AppExit>>,
){
    let mut binding = state_world.query::<&mut GameState>();
    let (_, mut state) = binding.iter().next().unwrap();
    
    match *state {
        GameState::MainMenu(_, _) => {},
        _ => return,
    }
    
    if let Some(ctx) = egui_ctx.read() {
        
        gui::SidePanel::left("my_panel").show(&ctx, |ui| {
            
            ui.heading("Komunterio");
            
            if ui.button("Ludi").clicked() {
                if let GameState::MainMenu(mut i, _) = *state {
                    i = true;
                }
            }
            
            if ui.button("Agordoj").clicked() {
                if let GameState::MainMenu(_, mut i) = *state {
                    i = true;
                }
            }
            
            if ui.button("Eliri").clicked() {
                app_exit.send(AppExit);
            }
            
        });
        
    }
}
