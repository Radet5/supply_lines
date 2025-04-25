use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, egui};

use crate::{age::Age, needs::Satiety, time_control::TimeController};

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .insert_resource(PickedGuy { entity: None })
            .add_plugins(MeshPickingPlugin)
            .add_systems(Update, ui_example_system);
    }
}

fn ui_example_system(
    mut contexts: EguiContexts,
    time_controller: Res<TimeController>,
    picked_guy: Res<PickedGuy>,
    query: Query<(&Age, &Satiety, &Name)>,
) {
    egui::Window::new("World Time").show(contexts.ctx_mut(), |ui| {
        ui.label(time_controller.simulated_elapsed_time_string());
    });

    if let Some(guy) = picked_guy.entity {
        if let Ok((age, satiety, name)) = query.get(guy) {
            egui::Window::new("Guy").show(contexts.ctx_mut(), |ui| {
                let guy_str = format!(
                    "{}: {}\nAge: {}\nSatiety: {{ value: {:05.2}, per_min: {}}}",
                    name,
                    guy,
                    age.formatted_age_string(&time_controller),
                    satiety.value,
                    satiety.drain_speed_per_sec,
                );
                ui.label(guy_str);
            });
        }
    }

    // let mut count = 0;
    // for _ in query.iter() {
    //     count += 1;
    // }
    // let count = format!("Aged Entities: {count}");
    // ui.label(&count);
    // println!("{count}");
}

#[derive(Resource, Debug)]
pub struct PickedGuy {
    entity: Option<Entity>,
}

pub fn pick_guy(trigger: Trigger<Pointer<Click>>, mut picked_guy: ResMut<PickedGuy>) {
    picked_guy.entity = Some(trigger.entity());
}
