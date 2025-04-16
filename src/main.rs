mod camera;
mod ground;

use bevy::prelude::*;
use bevy_rts_camera::RtsCameraPlugin;
use camera::CameraPlugin;
use ground::GroundPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0., 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(GroundPlugin)
        .add_plugins(RtsCameraPlugin)
        .add_plugins(CameraPlugin)
        .run();
}
