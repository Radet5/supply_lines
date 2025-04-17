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
        RtsCamera::default(),
        RtsCameraControls {
            key_up: KeyCode::KeyW,
            key_down: KeyCode::KeyS,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            edge_pan_width: 0.0,
            ..Default::default()
        },
    ));
}
