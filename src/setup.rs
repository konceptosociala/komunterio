use sonja::prelude::*;

use crate::{
    ui::{
        set_gui_style, 
        menu::{
            ChapterSelectDialog, 
            SaveLoadDialog, 
            SettingsDialog
        }, 
        settings::{Settings, ApplySettings, GuiScale},
        set_fonts_with_scale,
    }, 
    game::state::GameState,
};

pub struct SetupExtension;

impl Extension for SetupExtension {
    fn apply(&self, app: &mut Sonja) {
        app
            .add_setup_system(main_setup)
            .add_system(apply_settings)
            .add_system(set_gui_style);
    }
}

fn main_setup(
    mut cmd: Write<CommandBuffer>,
) -> SonjaResult<()> {    
    cmd.spawn((Settings::load("config/settings.toml")?, ApplySettings));
    
    cmd.spawn((GameState::MainMenu(
        ChapterSelectDialog::default(),
        SaveLoadDialog::default(),
        SettingsDialog::default(),
    ),));

    Ok(())
}

fn apply_settings(
    settings_world: SubWorld<(&Settings, &ApplySettings)>,
    renderer: Read<Renderer>,
    events: Read<Events>,
    mut cmd: Write<CommandBuffer>,
){
    let mut settings = settings_world.query::<(&Settings, &ApplySettings)>();
    let gui_ctx = events.get_handler::<GuiContext>().unwrap();

    if let Some((e, (settings, _))) = settings.iter().next() {
        cmd.remove_one::<ApplySettings>(e);

        // Set window mode
        let window = renderer.get_window();
        window.lock().unwrap().set_fullscreen(match settings.video.fullscreen {
            true => Some(WinitFullscreen::Borderless(None)),
            false => None,
        });

        // Set resolution

        // Set gui scale
        if let Some(ctx) = gui_ctx.read() {
            set_fonts_with_scale(&ctx, match settings.video.gui_scale {
                GuiScale::Large => 1.5,
                GuiScale::Normal => 1.25,
                GuiScale::Tiny => 1.0,
            });
        }
    }
}