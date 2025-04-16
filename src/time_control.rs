use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct TimeController {
    speed_factor: f32,
    scaled_delta: Duration,
    simulated_elapsed_secs: f64,
}

impl TimeController {
    pub fn scaled_delta(&self) -> Duration {
        self.scaled_delta
    }

    pub fn simulated_elapsed_secs(&self) -> f64 {
        self.simulated_elapsed_secs
    }
}

pub struct TimeControlPlugin;

impl Plugin for TimeControlPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimeController {
            speed_factor: 1.,
            scaled_delta: Duration::default(),
            simulated_elapsed_secs: 0.,
        })
        .add_systems(Update, (control_time_speed, scale_time));
    }
}

fn control_time_speed(
    mut time_controller: ResMut<TimeController>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Digit1) {
        time_controller.speed_factor = 1.;
        println!("Speed 1s/s");
    } else if keyboard_input.pressed(KeyCode::Digit2) {
        time_controller.speed_factor = 60.;
        println!("Speed 1min/s");
    } else if keyboard_input.pressed(KeyCode::Digit3) {
        time_controller.speed_factor = 60.0 * 60.0;
        println!("Speed 1hr/s");
    } else if keyboard_input.pressed(KeyCode::Digit4) {
        time_controller.speed_factor = 60.0 * 60.0 * 24.0;
        println!("Speed 1day/s");
    } else if keyboard_input.pressed(KeyCode::Backquote) {
        time_controller.speed_factor = 0.;
    }
}

fn scale_time(time: Res<Time>, mut time_controller: ResMut<TimeController>) {
    let speed_factor = time_controller.speed_factor;
    if speed_factor == 0.0 {
        return;
    }
    let scaled_delta_secs = time.delta_secs_f64() * time_controller.speed_factor as f64;

    time_controller.simulated_elapsed_secs += scaled_delta_secs;

    time_controller.scaled_delta = Duration::from_secs_f64(scaled_delta_secs);
}
