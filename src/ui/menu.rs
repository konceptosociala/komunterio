use std::fs::File;
use std::io::Write as IoWrite;
use sonja::prelude::*;

use crate::game::state::*;
use crate::ui::settings::Settings;

use super::settings::{Resolution, GuiScale, ApplySettings};

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
    fn apply(&self, app: &mut Sonja) {
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
    renderer: Read<Renderer>,
    mut cmd: Write<CommandBuffer>,
){
    let mut state = state_world.query::<&mut GameState>();
    let (_, mut state) = state.iter().next().unwrap();

    let mut settings = settings_world.query::<&mut Settings>();
    let (e, mut settings) = settings.iter().next().unwrap();
    
    match *state {
        GameState::MainMenu(_, _, _) => {},
        _ => return,
    }

    let window = renderer.get_window();
    let size = window.lock().unwrap().inner_size();
    
    let gui_ctx = events.get_handler::<GuiContext>().unwrap();
    let mut app_exit = events.get_handler::<AppExit>().unwrap();
    if let Some(ctx) = gui_ctx.read() {
        if ctx.input().key_pressed(gui::Key::Escape) {
            if let GameState::MainMenu(ref mut p, ref mut l, ref mut s) = *state {
                p.is_active = false;
                l.is_active = false;
                s.is_active = false;
            }
        }
        
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
                .pivot(gui::Align2::CENTER_CENTER)
                .open(&mut select_dialog.is_active)
                .fixed_pos(gui::pos2(
                    (size.width / 2) as f32,
                    (size.height / 2) as f32
                ))
                .show(&ctx, |ui| 
            {
                ui.label("Hello World!");
            });                
        }
        
        // Game settings dialog
        if let GameState::MainMenu(_, _, ref mut settings_dialog) = *state {              
            gui::Window::new("Agordoj")
                .collapsible(false)
                .pivot(gui::Align2::CENTER_CENTER)
                .fixed_size(gui::vec2(0.0, 0.0))
                .open(&mut settings_dialog.is_active)
                .fixed_pos(gui::pos2(
                    (size.width / 2) as f32,
                    (size.height / 2) as f32
                ))
                .show(&ctx, |ui| 
            {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut settings_dialog.tab, SettingsTab::Video, "Video");
                    ui.selectable_value(&mut settings_dialog.tab, SettingsTab::Sound, "Sound");
                    ui.selectable_value(&mut settings_dialog.tab, SettingsTab::Game, "Game");
                    ui.selectable_value(&mut settings_dialog.tab, SettingsTab::Controls, "Controls");
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.columns(2, |cols|{
                        cols[0].label("Resolution");
                        gui::ComboBox::from_id_source("select_resolution")
                            .selected_text(settings.video.resolution.to_string())
                            .show_ui(&mut cols[1], |ui| {
                                ui.selectable_value(&mut settings.video.resolution, Resolution::System, "System");
                                ui.selectable_value(&mut settings.video.resolution, Resolution::R1920x1080, "1920x1080");
                                ui.selectable_value(&mut settings.video.resolution, Resolution::R1280x720, "1280x720");
                                ui.selectable_value(&mut settings.video.resolution, Resolution::R1024x576, "1024x576");
                                ui.selectable_value(&mut settings.video.resolution, Resolution::R640x360, "640x360");
                            }
                        );
                    });
                });

                ui.horizontal(|ui| {
                    ui.columns(2, |cols|{
                        cols[0].label("GUI Scale");
                        gui::ComboBox::from_id_source("select_gui_scaling")
                            .selected_text(settings.video.gui_scale.to_string())
                            .show_ui(&mut cols[1], |ui| {
                                ui.selectable_value(&mut settings.video.gui_scale, GuiScale::Large, "Large");
                                ui.selectable_value(&mut settings.video.gui_scale, GuiScale::Normal, "Normal");
                                ui.selectable_value(&mut settings.video.gui_scale, GuiScale::Tiny, "Tiny");
                            }
                        );
                    });
                });

                ui.horizontal(|ui| {
                    ui.columns(2, |cols|{
                        cols[0].label("Fullscreen");
                        cols[1].checkbox(&mut settings.video.fullscreen, "");
                    });
                });

                if ui.button("Apply").clicked() {
                    cmd.insert_one(e, ApplySettings);

                    let mut file = File::create("config/settings.toml")
                        .expect("Cannot open config file!");
                    
                    let ser = toml::to_string_pretty(&*settings).unwrap();
                    file.write(ser.as_bytes()).unwrap_or_else(|_|{
                        error!("Cannot write config to file!");
                        return 0;
                    });
                }
            });                
        }
    }
}
