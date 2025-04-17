use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, egui};

use crate::time_control::TimeController;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, ui_example_system);
    }
}

fn ui_example_system(mut contexts: EguiContexts, time_controller: Res<TimeController>) {
    egui::Window::new("World Time").show(contexts.ctx_mut(), |ui| {
        ui.label(time_controller.simulated_elapsed_time_string());
    });
}
