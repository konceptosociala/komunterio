use serde::{Serialize, Deserialize};
use despero::prelude::*;

use crate::game::state::*;
use crate::ui::settings::Settings;

#[derive(Default, Clone, Copy, Debug)]
pub struct ChapterSelectDialog {
    pub is_active: bool,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct SaveLoadDialog {
    pub is_active: bool,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct SettingsDialog {
    pub is_active: bool,
    pub tab: SettingsTab,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum SettingsTab {
    #[default]
    Video,
    Sound,
    Game,
    Controls,
}

#[derive(Clone, Copy, Debug)]
pub struct MainMenuExtension;

impl Extension for MainMenuExtension {
    fn apply(&self, app: &mut Despero) {
        app
            .add_setup_system(setup_main_menu)
            .add_system(main_menu_system);
    }
}

fn setup_main_menu(
    mut _cmd: Write<CommandBuffer>,
){
    
}

fn main_menu_system(
    state_world: SubWorld<&mut GameState>,
    settings_world: SubWorld<&mut Settings>,
    events: Read<Events>,
){
    let mut state = state_world.query::<&mut GameState>();
    let (_, mut state) = state.iter().next().unwrap();

    let mut settings = settings_world.query::<&mut Settings>();
    let (_, mut settings) = settings.iter().next().unwrap();
    
    match *state {
        GameState::MainMenu(_, _, _) => {},
        _ => return,
    }
    
    let gui_ctx = events.get_handler::<GuiContext>().unwrap();
    let mut app_exit = events.get_handler::<AppExit>().unwrap();
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
                    select_dialog.is_active = true;
                }
            }
            
            if ui.add(gui::Button::new("Ŝarĝi").frame(false)).clicked() {
                if let GameState::MainMenu(_, ref mut load_dialog, _) = *state {
                    load_dialog.is_active = true;
                }
            }
            
            if ui.add(gui::Button::new("Agordoj").frame(false)).clicked() {
                if let GameState::MainMenu(_, _, ref mut settings_dialog) = *state {
                    settings_dialog.is_active = true;
                }
            }
            
            if ui.add(gui::Button::new("Eliri").frame(false)).clicked() {
                app_exit.send(AppExit);
            }
            
        });
        
        // Select level dialog
        if let GameState::MainMenu(ref mut select_dialog, _, _) = *state {              
            gui::Window::new("Ĉapitrelekta dialogo")
                .collapsible(false)
                .open(&mut select_dialog.is_active).show(&ctx, |ui| 
            {
            
                ui.label("Hello World!");
               
            });                
        }
        
        // Game settings dialog
        if let GameState::MainMenu(_, _, ref mut settings_dialog) = *state {              
            gui::Window::new("Agordoj")
                .collapsible(false)
                .open(&mut settings_dialog.is_active).show(&ctx, |ui| 
            {
            
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut settings_dialog.tab, SettingsTab::Video, "Video");
                    ui.selectable_value(&mut settings_dialog.tab, SettingsTab::Sound, "Sound");
                    ui.selectable_value(&mut settings_dialog.tab, SettingsTab::Game, "Game");
                    ui.selectable_value(&mut settings_dialog.tab, SettingsTab::Controls, "Controls");
                });

                ui.separator();

                gui::Frame::none()
                    .stroke(gui::Stroke::new(1.0, gui::Color32::DARK_GRAY))
                    .inner_margin(5.0)
                    .show(ui, |ui| 
                {
                    ui.label("Label with red background");
                });
               
            });                
        }
    }
}
