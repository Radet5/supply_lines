use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct TimeController {
    speed_factor: u8,
}

impl TimeController {
    pub fn scale_duration(&self, duration: &Duration) -> Duration {
        let micros = duration.as_micros() as u64;
        let speed: u64 = self.speed_factor as u64;
        Duration::from_micros(micros * speed)
    }
}

pub struct TimeControlPlugin;

impl Plugin for TimeControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimeController { speed_factor: 1 })
            .add_systems(Update, control_time_speed);
    }
}

fn control_time_speed(
    mut time_controller: ResMut<TimeController>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Digit1) {
        time_controller.speed_factor = 1;
    } else if keyboard_input.pressed(KeyCode::Digit2) {
        time_controller.speed_factor = 2;
    } else if keyboard_input.pressed(KeyCode::Digit3) {
        time_controller.speed_factor = 3;
    } else if keyboard_input.pressed(KeyCode::Backquote) {
        time_controller.speed_factor = 0;
    }
}
