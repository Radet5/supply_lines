use bevy::{input::keyboard::KeyboardInput, prelude::*};
use rand::Rng;

use crate::{
    age::Age,
    asset_loader::{AnimationData, SceneAssets, asset_load_handle},
    movement::{Destination, Speed, animate_movement, idle_on_stop, on_arrive},
    time_control::TimeController,
};

pub struct AnimalPlugin;

impl Plugin for AnimalPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnAnimalEvent>()
            .add_systems(Startup, spawn_animals)
            .add_systems(
                FixedUpdate,
                goto_random
                    .run_if(|keys: Res<ButtonInput<KeyCode>>| keys.just_pressed(KeyCode::KeyG)),
            )
            .add_systems(FixedUpdate, spawn_animal);
    }
}

static ANIMAL_CONFIG: AnimalConfig = AnimalConfig { initial_count: 4 };

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

fn goto_random(mut commands: Commands, query: Query<Entity, (With<Animal>, Without<Destination>)>) {
    let mut rng = rand::rng();
    for entity in query.iter() {
        let destination = Vec3::new(
            rng.random_range(-20.0..20.0),
            0.,
            rng.random_range(-20.0..20.0),
        );
        commands
            .entity(entity)
            .insert(Destination::new(destination));
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
        match spawn_event.animal_type {
            AnimalType::Deer => {
                if rng.random_bool(0.5) {
                    name = "deer";
                    asset = scene_assets.deer.clone();
                    animation_index = 0;
                } else {
                    name = "stag";
                    asset = scene_assets.stag.clone();
                    animation_index = 0;
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
            .observe(asset_load_handle)
            .observe(animate_movement)
            .observe(idle_on_stop)
            .observe(on_arrive);
    }
}

struct AnimalConfig {
    initial_count: u32,
}
