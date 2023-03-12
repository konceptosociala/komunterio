use despero::prelude::*;

use crate::game::state::*;

pub fn main_menu_system(
    state_world: SubWorld<&mut GameState>,
    gui_ctx: Read<EventHandler<GuiContext>>,
    mut app_exit: Write<EventHandler<AppExit>>,
){
    let mut binding = state_world.query::<&mut GameState>();
    let (_, mut state) = binding.iter().next().unwrap();
    
    match *state {
        GameState::MainMenu(_, _, _) => {},
        _ => return,
    }
    
    if let Some(ctx) = gui_ctx.read() {
        
        gui::SidePanel::left("Main menu panel")
            .default_width(300.0)
            .min_width(300.0)
            .max_width(300.0)
            .resizable(false)
            .show(&ctx, |ui| 
        {
            
            ui.label(gui::RichText::new("Komunterio").font(gui::FontId::proportional(35.0)));
            ui.separator();
            
            if ui.add(gui::Button::new("Ludi").frame(false)).clicked() {
                if let GameState::MainMenu(ref mut select_dialog, _, _) = *state {
                    *select_dialog = true;
                }
            }
            
            if ui.add(gui::Button::new("Ŝarĝi").frame(false)).clicked() {
                if let GameState::MainMenu(_, ref mut load_dialog, _) = *state {
                    *load_dialog = true;
                }
            }
            
            if ui.add(gui::Button::new("Agordoj").frame(false)).clicked() {
                if let GameState::MainMenu(_, _, ref mut settings_dialog) = *state {
                    *settings_dialog = true;
                }
            }
            
            if ui.add(gui::Button::new("Eliri").frame(false)).clicked() {
                app_exit.send(AppExit);
            }
            
        });
        
        // Select level dialog
        if let GameState::MainMenu(ref mut select_dialog, _, _) = *state {              
            gui::Window::new("Select chapter dialog").open(select_dialog).show(&ctx, |ui| {
            
                ui.label("Hello World!");
               
            });                
        }
        
    }
}
