use bevy::prelude::*;
use rand::Rng;
use std::ops::Range;

use crate::{
    age::Age, asset_loader::SceneAssets, schedule::StartupSet, time_control::TimeController,
};

const TREE_CONFIG: TreeConfig = TreeConfig {
    spawn_range_x: -20.0..20.0,
    spawn_range_z: -20.0..20.0,
    grow_check_seconds: 0.5,
    maturity_seconds: 60. * 60. * 24. * 5.,
    lifespan_days: 300.,
    min_dist_between_trees_sqrd: 2.8 * 2.8,
    initial_tree_count: 100,
    scale: 0.1,
};

const FRUIT_CONFIG: FruitConfig = FruitConfig {
    spawn_check_sim_seconds: 60. * 60. * 24.,
    decay_check_sim_seconds: 60. * 60. * 24.,
    lifespan_days: 30.,
    scale: 0.25,
    spawn_count_range: 1..4,
};

const DAILY_FRUIT_PROBABILITY: f64 = 0.1;
const TREE_SPAWN_FROM_FRUIT_PROBABILITY: f64 = 0.2;

#[derive(Component, Debug)]
pub struct Tree;

#[derive(Event, Debug)]
struct SpawnTreeEvent {
    translation: Option<Vec3>,
}

impl SpawnTreeEvent {
    fn new(translation: Option<Vec3>) -> Self {
        Self { translation }
    }
}

#[derive(Component, Debug)]
pub struct Fruit;

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
        app.insert_resource(GrowTimer {
            timer: Timer::from_seconds(TREE_CONFIG.grow_check_seconds, TimerMode::Repeating),
        })
        .insert_resource(FruitTimer {
            timer: Timer::from_seconds(FRUIT_CONFIG.spawn_check_sim_seconds, TimerMode::Repeating),
        })
        .insert_resource(DecayTimer {
            timer: Timer::from_seconds(FRUIT_CONFIG.decay_check_sim_seconds, TimerMode::Repeating),
        })
        .add_event::<SpawnTreeEvent>()
        .add_systems(Startup, spawn_trees.in_set(StartupSet::StartupRoundB))
        .add_systems(FixedUpdate, grow)
        // .add_systems(FixedUpdate, spawn_trees)
        .add_systems(FixedUpdate, spawn_fruit)
        .add_systems(FixedUpdate, (decay_fruit, spawn_tree).chain());
    }
}

fn spawn_trees(mut spawn_tree_event_writer: EventWriter<SpawnTreeEvent>) {
    for _ in 0..TREE_CONFIG.initial_tree_count {
        spawn_tree_event_writer.send(SpawnTreeEvent::new(None));
    }
}

fn spawn_tree(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    time_controller: Res<TimeController>,
    mut spawn_tree_event_reader: EventReader<SpawnTreeEvent>,
) {
    let mut rng = rand::rng();

    for spawn_event in spawn_tree_event_reader.read() {
        let random_angle = rng.random_range(0.0..std::f32::consts::PI);
        let rotation: Quat = Quat::from_axis_angle(Vec3::new(0., 1., 0.), random_angle);

        let translation = spawn_event.translation;
        let translation = translation.unwrap_or(Vec3::new(
            rng.random_range(TREE_CONFIG.spawn_range_x),
            0.,
            rng.random_range(TREE_CONFIG.spawn_range_z),
        ));

        commands.spawn((
            Name::new("Tree"),
            SceneRoot(scene_assets.tree.clone()),
            Transform::from_translation(translation)
                .with_rotation(rotation)
                .with_scale(Vec3::splat(TREE_CONFIG.scale)),
            Tree,
            Age::new(&time_controller),
        ));
    }
}

fn decay_fruit(
    mut commands: Commands,
    query: Query<(Entity, &Age, &Transform), With<Fruit>>,
    tree_query: Query<&Transform, With<Tree>>,
    time_controller: Res<TimeController>,
    mut decay_timer: ResMut<DecayTimer>,
    mut spawn_tree_event_writer: EventWriter<SpawnTreeEvent>,
) {
    decay_timer.timer.tick(time_controller.scaled_delta());
    if !decay_timer.timer.just_finished() {
        return;
    }

    for (entity, age, transform) in query.iter() {
        if age.age_days(&time_controller) as f32 > FRUIT_CONFIG.lifespan_days {
            let mut rng = rand::rng();
            if rng.random_bool(TREE_SPAWN_FROM_FRUIT_PROBABILITY) {
                let mut dont: bool = false;
                for tree_transform in tree_query.iter() {
                    if (tree_transform
                        .translation
                        .distance_squared(transform.translation))
                        < TREE_CONFIG.min_dist_between_trees_sqrd
                    {
                        dont = true;
                    }
                }
                if !dont {
                    spawn_tree_event_writer.send(SpawnTreeEvent::new(Some(transform.translation)));
                }
            }
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
        if age.age_seconds(&time_controller) as f32 >= TREE_CONFIG.maturity_seconds {
            let mut rng = rand::rng();
            if rng.random_bool(DAILY_FRUIT_PROBABILITY) {
                let fruit_count = rng.random_range(FRUIT_CONFIG.spawn_count_range);
                for _ in 0..fruit_count {
                    let random_point = Vec3::new(
                        rng.random_range(-100.0..100.0),
                        0.,
                        rng.random_range(-100.0..100.0),
                    );
                    let distance = rng.random_range((0.5 * FRUIT_CONFIG.scale)..4.0);
                    let tree_point = transform.translation;
                    let fruit_point = tree_point
                        .move_towards(random_point, distance)
                        .with_y(0.25 * FRUIT_CONFIG.scale);
                    commands.spawn((
                        Name::new("Fruit"),
                        SceneRoot(scene_assets.fruit.clone()),
                        Transform::from_translation(fruit_point)
                            .with_scale(Vec3::splat(FRUIT_CONFIG.scale)),
                        Fruit,
                        Age::new(&time_controller),
                    ));
                }
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
            let growth_pct =
                age.age_seconds(&time_controller) as f32 / TREE_CONFIG.maturity_seconds;
            let amount = 0.1 + (0.9 * growth_pct).min(0.9);
            transform.scale = Vec3::splat(amount);
        }
        if age.age_days(&time_controller) as f32 > TREE_CONFIG.lifespan_days {
            // TODO: switch this to an event and then schedule despawns correctly
            commands.entity(entity).despawn_recursive();
        }
        // println!("Tree age: {}", age.formatted_age_string(&time_controller));
        // println!("Tree age: {}s", age.age_seconds(&time_controller));
    }
}

struct TreeConfig {
    initial_tree_count: u32,
    spawn_range_x: Range<f32>,
    spawn_range_z: Range<f32>,
    grow_check_seconds: f32,
    maturity_seconds: f32,
    lifespan_days: f32,
    min_dist_between_trees_sqrd: f32,
    scale: f32,
}

struct FruitConfig {
    spawn_check_sim_seconds: f32,
    decay_check_sim_seconds: f32,
    lifespan_days: f32,
    scale: f32,
    spawn_count_range: Range<u8>,
}
