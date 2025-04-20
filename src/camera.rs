use bevy::prelude::*;
use bevy_rts_camera::{RtsCamera, RtsCameraControls};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_rts_camera);
    }
}

fn spawn_rts_camera(mut commands: Commands) {
    commands.spawn((
        RtsCamera {
            height_max: 80.,
            height_min: 5.,
            min_angle: 0.65,
            ..Default::default()
        },
        RtsCameraControls {
            key_up: KeyCode::KeyW,
            key_down: KeyCode::KeyS,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            edge_pan_width: 0.0,
            zoom_sensitivity: 0.25,
            ..Default::default()
        },
    ));
}
