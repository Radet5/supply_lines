use avian3d::prelude::*;
use bevy::prelude::*;
use rand::Rng;

use crate::{
    age::Age,
    asset_loader::{AnimationData, SceneAssets, asset_load_handle},
    movement::{Destination, Speed, animate_movement, idle_on_stop, on_arrive},
    navigation::{EntityPath, FindPathEvent, NoPathFoundEvent, Obstacle},
    time_control::TimeController,
    vegetation::{Tree, spawn_trees, within_dist_sqrd_of_transforms},
};

pub struct AnimalPlugin;

impl Plugin for AnimalPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnAnimalEvent>()
            .add_systems(Startup, spawn_animals.after(spawn_trees))
            .add_systems(
                FixedUpdate,
                goto_random
                    .run_if(|keys: Res<ButtonInput<KeyCode>>| keys.just_pressed(KeyCode::KeyG)),
            )
            .add_systems(FixedUpdate, spawn_animal);
    }
}

static ANIMAL_CONFIG: AnimalConfig = AnimalConfig { initial_count: 10 };

#[derive(Component, Debug)]
pub struct Animal {
    animal_type: AnimalType,
}

#[derive(Clone, Debug)]
enum AnimalType {
    Deer,
}

#[derive(Event, Debug)]
struct SpawnAnimalEvent {
    animal_type: AnimalType,
    translation: Option<Vec3>,
}

impl SpawnAnimalEvent {
    fn new(animal_type: AnimalType, translation: Option<Vec3>) -> Self {
        Self {
            animal_type,
            translation,
        }
    }
}

fn spawn_animals(mut spawn_animal_event_writer: EventWriter<SpawnAnimalEvent>) {
    for _ in 0..ANIMAL_CONFIG.initial_count {
        spawn_animal_event_writer.send(SpawnAnimalEvent::new(AnimalType::Deer, None));
    }
}

fn unstuck_animals(
    trigger: Trigger<NoPathFoundEvent>,
    mut commands: Commands,
    query: Query<&Transform, (With<Animal>, Without<Destination>)>,
    obstacle_query: Query<&Transform, With<Tree>>,
) {
    let standoff_dist = 1.5;
    if let Ok(animal_transform) = query.get(trigger.entity()) {
        if let Some(obstacle_translation) = within_dist_sqrd_of_transforms(
            0.5,
            obstacle_query.iter(),
            &animal_transform.translation,
        ) {
            let mut dir = animal_transform.translation - obstacle_translation;
            dir.y = 0.0;

            if dir.length_squared() > 0.0 {
                println!("Unstucking: {:?}", trigger.entity());
                let flee_dir = dir.normalize(); // unit vector away
                // multiply by your standoff distance
                let mut target = animal_transform.translation + flee_dir * standoff_dist;
                target.y = animal_transform.translation.y;
                commands
                    .entity(trigger.entity())
                    .insert(Destination::new(target));
            } else {
                println!("NONONO");
            }
        }
    }
}

fn goto_random(
    // mut commands: Commands,
    query: Query<(Entity, &Transform), (With<Animal>, (Without<Destination>, Without<EntityPath>))>,
    obstacle_query: Query<&Transform, With<Obstacle>>,
    mut find_path_event_writer: EventWriter<FindPathEvent>,
) {
    let mut rng = rand::rng();
    for (entity, transform) in query.iter() {
        let to_point = Vec3::new(
            rng.random_range(-20.0..20.0),
            0.0,
            rng.random_range(-20.0..20.0),
        );

        // let from_point = Vec2::new(transform.translation.x, transform.translation.z);
        find_path_event_writer.send(FindPathEvent::new(transform.translation, to_point, entity));
        // let to_point = Vec3::new(
        //     rng.random_range(-20.0..20.0),
        //     0.,
        //     rng.random_range(-20.0..20.0),
        // );
        // commands.entity(entity).insert(Destination::new(to_point));
    }
}

fn spawn_animal(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    time_controller: Res<TimeController>,
    mut spawn_animal_event_reader: EventReader<SpawnAnimalEvent>,
    // mut players: Query<&mut AnimationPlayer>,
) {
    let mut rng = rand::rng();

    for spawn_event in spawn_animal_event_reader.read() {
        let random_angle = rng.random_range(0.0..std::f32::consts::PI);
        let rotation: Quat = Quat::from_axis_angle(Vec3::new(0., 1., 0.), random_angle);

        let translation = spawn_event.translation;
        let translation = translation.unwrap_or(Vec3::new(
            rng.random_range(-10.0..10.),
            0.,
            rng.random_range(-10.0..10.),
        ));

        let name;
        let asset;
        let speed;
        let animation_index;
        let collider_stuff;
        match spawn_event.animal_type {
            AnimalType::Deer => {
                if rng.random_bool(0.5) {
                    name = "deer";
                    asset = scene_assets.deer.clone();
                    animation_index = 0;
                    collider_stuff = (
                        Collider::cuboid(0.25, 1.0, 1.1),
                        Transform::from_translation(Vec3::new(0.0, 2.25, 0.5)),
                    );
                } else {
                    name = "stag";
                    asset = scene_assets.stag.clone();
                    animation_index = 0;
                    collider_stuff = (
                        Collider::cuboid(0.35, 1.3, 1.1),
                        Transform::from_translation(Vec3::new(0.0, 2.30, 0.5)),
                    );
                }
                speed = 1.125;
            }
        }

        commands
            .spawn((
                Name::new(name),
                SceneRoot(asset),
                Transform::from_translation(translation)
                    .with_rotation(rotation)
                    .with_scale(Vec3::splat(0.25)),
                Animal {
                    animal_type: spawn_event.animal_type.clone(),
                },
                Age::new(&time_controller),
                Speed::new(speed),
                AnimationData {
                    animation_key: name,
                    animation_index,
                },
            ))
            .with_children(|parent| {
                parent.spawn(collider_stuff);
            })
            .observe(unstuck_animals)
            .observe(asset_load_handle)
            .observe(animate_movement)
            .observe(idle_on_stop)
            .observe(on_arrive);
    }
}

struct AnimalConfig {
    initial_count: u32,
}
