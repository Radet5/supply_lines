use bevy::prelude::*;
use rand::Rng;
use std::ops::Range;

use crate::{asset_loader::SceneAssets, schedule::StartupSet, time_control::TimeController};

const SPAWN_RANGE_X: Range<f32> = -10.0..10.0;
const SPAWN_RANGE_Z: Range<f32> = -10.0..10.0;
const SPAWN_TIME_SECONDS: f32 = 60.;

#[derive(Component, Debug)]
pub struct Tree;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

pub struct VegetationPlugin;

impl Plugin for VegetationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .add_systems(Startup, spawn_tree.in_set(StartupSet::StartupRoundB))
        .add_systems(FixedUpdate, spawn_trees);
    }
}

fn spawn_trees(
    commands: Commands,
    time_controller: Res<TimeController>,
    mut spawn_timer: ResMut<SpawnTimer>,
    scene_assets: Res<SceneAssets>,
) {
    spawn_timer.timer.tick(time_controller.scaled_delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }
    spawn_tree(commands, scene_assets);
}

fn spawn_tree(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    let mut rng = rand::rng();

    let random_angle = rng.random_range(0.0..std::f32::consts::PI);
    let rotation: Quat = Quat::from_axis_angle(Vec3::new(0., 1., 0.), random_angle);

    let translation = Vec3::new(
        rng.random_range(SPAWN_RANGE_X),
        0.,
        rng.random_range(SPAWN_RANGE_Z),
    );

    commands.spawn((
        SceneRoot(scene_assets.tree.clone()),
        Transform::from_translation(translation).with_rotation(rotation),
        Tree,
    ));
}
