mod age;
mod animal;
mod asset_loader;
mod camera;
mod debug;
mod ground;
mod hud;
mod light;
mod movement;
mod navigation;
mod needs;
mod schedule;
mod time_control;
mod vegetation;

use age::Age;
use animal::AnimalPlugin;
use asset_loader::AssetLoaderPlugin;
use avian3d::prelude::*;
use bevy::{log::LogPlugin, prelude::*};
use bevy_rts_camera::RtsCameraPlugin;
use camera::CameraPlugin;
use debug::DebugPlugin;
use ground::GroundPlugin;
use hud::HUDPlugin;
use light::LightPlugin;
use movement::MovementPlugin;
use navigation::NavigationPlugin;
use needs::NeedsPlugin;
use schedule::SchedulePlugin;
use time_control::TimeControlPlugin;
use vegetation::VegetationPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.529, 0.808, 0.922)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 75.,
        })
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "big_brain=debug,statiety=debug".to_string(),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(NavigationPlugin)
        .add_plugins(NeedsPlugin)
        .add_plugins(LightPlugin)
        .add_plugins(SchedulePlugin)
        .add_plugins(GroundPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(RtsCameraPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(VegetationPlugin)
        .add_plugins(AnimalPlugin)
        .add_plugins(TimeControlPlugin)
        .add_plugins(HUDPlugin)
        .add_plugins(DebugPlugin)
        .register_type::<Age>()
        .run();
}
