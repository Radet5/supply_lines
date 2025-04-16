use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct TimeController {
    speed_factor: f32,
    scaled_delta: Duration,
}

impl TimeController {
    pub fn scale_duration(&self, duration: &Duration) -> Duration {
        let micros = duration.as_micros() as f32;
        let scaled_micros = micros * self.speed_factor;
        Duration::from_micros(scaled_micros as u64)
    }

    pub fn scaled_delta(&self) -> Duration {
        self.scaled_delta
    }
}

pub struct TimeControlPlugin;

impl Plugin for TimeControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimeController {
            speed_factor: 1.,
            scaled_delta: Duration::default(),
        })
        .add_systems(Update, (control_time_speed, scale_delta));
    }
}

fn control_time_speed(
    mut time_controller: ResMut<TimeController>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Digit1) {
        time_controller.speed_factor = 1.;
    } else if keyboard_input.pressed(KeyCode::Digit2) {
        time_controller.speed_factor = 30.;
    } else if keyboard_input.pressed(KeyCode::Digit3) {
        time_controller.speed_factor = 60.;
    } else if keyboard_input.pressed(KeyCode::Backquote) {
        time_controller.speed_factor = 0.;
    }
}

fn scale_delta(time: Res<Time>, mut time_controller: ResMut<TimeController>) {
    time_controller.scaled_delta = time_controller.scale_duration(&time.delta());
}
