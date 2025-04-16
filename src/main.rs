mod asset_loader;
mod camera;
mod ground;
mod schedule;
mod vegetation;

use asset_loader::AssetLoaderPlugin;
use bevy::prelude::*;
use bevy_rts_camera::RtsCameraPlugin;
use camera::CameraPlugin;
use ground::GroundPlugin;
use schedule::SchedulePlugin;
use vegetation::VegetationPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.529, 0.808, 0.922)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(SchedulePlugin)
        .add_plugins(GroundPlugin)
        .add_plugins(RtsCameraPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(VegetationPlugin)
        .run();
}
