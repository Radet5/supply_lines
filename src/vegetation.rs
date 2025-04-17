use bevy::prelude::*;
use rand::Rng;
use std::ops::Range;

use crate::{
    age::Age, asset_loader::SceneAssets, schedule::StartupSet, time_control::TimeController,
};

const SPAWN_RANGE_X: Range<f32> = -10.0..10.0;
const SPAWN_RANGE_Z: Range<f32> = -10.0..10.0;
const SPAWN_TIME_SECONDS: f32 = 60. * 60. * 24. * 30.;
const GROW_CHECK_SECONDS: f32 = 0.5;
const MATURITY_SECONDS: f32 = 60. * 60. * 24. * 5.;
const TREE_LIFESPAN_DAYS: f32 = 300.;

const FRUIT_SPAWN_CHECK_SIM_SECONDS: f32 = 60. * 60. * 24.;
const FRUIT_DECAY_CHECK_SIM_SECONDS: f32 = 60. * 60. * 24.;
const FRUIT_LIFESPAN_DAYS: f32 = 30.;
const FRUIT_SCALE: f32 = 0.5;
const DAILY_FRUIT_PROBABILITY: f64 = 0.1;

#[derive(Component, Debug)]
pub struct Tree;

#[derive(Component, Debug)]
pub struct Fruit;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

#[derive(Resource, Debug)]
pub struct GrowTimer {
    timer: Timer,
}

#[derive(Resource, Debug)]
pub struct FruitTimer {
    timer: Timer,
}

#[derive(Resource, Debug)]
pub struct DecayTimer {
    timer: Timer,
}

pub struct VegetationPlugin;

impl Plugin for VegetationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        })
        .insert_resource(GrowTimer {
            timer: Timer::from_seconds(GROW_CHECK_SECONDS, TimerMode::Repeating),
        })
        .insert_resource(FruitTimer {
            timer: Timer::from_seconds(FRUIT_SPAWN_CHECK_SIM_SECONDS, TimerMode::Repeating),
        })
        .insert_resource(DecayTimer {
            timer: Timer::from_seconds(FRUIT_DECAY_CHECK_SIM_SECONDS, TimerMode::Repeating),
        })
        .add_systems(Startup, spawn_tree.in_set(StartupSet::StartupRoundB))
        .add_systems(FixedUpdate, grow)
        .add_systems(FixedUpdate, spawn_trees)
        .add_systems(FixedUpdate, spawn_fruit)
        .add_systems(FixedUpdate, decay_fruit);
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
    spawn_tree(commands, scene_assets, time_controller);
}

fn spawn_tree(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    time_controller: Res<TimeController>,
) {
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
        Transform::from_translation(translation)
            .with_rotation(rotation)
            .with_scale(Vec3::splat(0.1)),
        Tree,
        Age::new(&time_controller),
    ));
}

fn decay_fruit(
    mut commands: Commands,
    query: Query<(Entity, &Age), With<Fruit>>,
    time_controller: Res<TimeController>,
    mut decay_timer: ResMut<DecayTimer>,
) {
    decay_timer.timer.tick(time_controller.scaled_delta());
    if !decay_timer.timer.just_finished() {
        return;
    }

    for (entity, age) in query.iter() {
        if age.age_days(&time_controller) as f32 > FRUIT_LIFESPAN_DAYS {
            // TODO: switch this to an event and then schedule despawns correctly
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn spawn_fruit(
    time_controller: Res<TimeController>,
    mut query: Query<(&Transform, &Age), With<Tree>>,
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    mut fruit_timer: ResMut<FruitTimer>,
) {
    fruit_timer.timer.tick(time_controller.scaled_delta());
    if !fruit_timer.timer.just_finished() {
        return;
    }

    for (transform, age) in query.iter_mut() {
        if age.age_seconds(&time_controller) as f32 >= MATURITY_SECONDS {
            let mut rng = rand::rng();
            if rng.random_bool(DAILY_FRUIT_PROBABILITY) {
                let random_point = Vec3::new(
                    rng.random_range(-100.0..100.0),
                    0.,
                    rng.random_range(-100.0..100.0),
                );
                let distance = rng.random_range((0.25 * FRUIT_SCALE)..3.0);
                let tree_point = transform.translation;
                let fruit_point = tree_point
                    .move_towards(random_point, distance)
                    .with_y(0.25 * FRUIT_SCALE);
                commands.spawn((
                    SceneRoot(scene_assets.fruit.clone()),
                    Transform::from_translation(fruit_point).with_scale(Vec3::splat(FRUIT_SCALE)),
                    Fruit,
                    Age::new(&time_controller),
                ));
            }
        }
    }
}

fn grow(
    mut query: Query<(Entity, &mut Transform, &Age), With<Tree>>,
    time_controller: Res<TimeController>,
    mut grow_timer: ResMut<GrowTimer>,
    time: Res<Time>,
    mut commands: Commands,
) {
    grow_timer.timer.tick(time.delta());
    if !grow_timer.timer.just_finished() {
        return;
    }

    for (entity, mut transform, age) in query.iter_mut() {
        if transform.scale.x < 1.0 {
            let growth_pct = age.age_seconds(&time_controller) as f32 / MATURITY_SECONDS;
            let amount = 0.1 + (0.9 * growth_pct).min(0.9);
            transform.scale = Vec3::splat(amount);
        }
        if age.age_days(&time_controller) as f32 > TREE_LIFESPAN_DAYS {
            // TODO: switch this to an event and then schedule despawns correctly
            commands.entity(entity).despawn_recursive();
        }
        // println!("Tree age: {}", age.formatted_age_string(&time_controller));
        // println!("Tree age: {}s", age.age_seconds(&time_controller));
    }
}
