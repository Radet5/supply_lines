use bevy::prelude::*;

use crate::{asset_loader::AnimationData, time_control::TimeController};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Destination>()
            .register_type::<Speed>()
            .add_systems(FixedUpdate, update_position);
    }
}

#[derive(Component, Debug, Reflect)]
pub struct Destination {
    value: Vec3,
}

impl Destination {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug, Reflect)]
pub struct Speed {
    value: f32,
}

impl Speed {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

#[derive(Event, Debug)]
pub struct ArrivedEvent;

pub fn on_arrive(trigger: Trigger<ArrivedEvent>, mut query: Query<Entity>, mut commands: Commands) {
    if let Ok(entity) = query.get_mut(trigger.entity()) {
        commands.entity(entity).remove::<Destination>();
    }
}

fn update_position(
    time_control: Res<TimeController>,
    mut query: Query<(&mut Transform, &Destination, &Speed, Entity)>,
    mut commands: Commands,
) {
    for (mut transform, destination, speed, entity) in query.iter_mut() {
        if transform.translation.distance_squared(destination.value) > 1. {
            let delta = time_control.scaled_delta().as_secs_f32();
            let one_eighty: f32 = 180.0;
            let one_eighty = one_eighty.to_radians();
            let mut looking_at = transform.looking_at(destination.value, Dir3::Y);
            looking_at.rotate_y(one_eighty);
            let angle: f32 = 0.1 * delta;
            let angle = angle.to_degrees();
            let rotation = transform
                .rotation
                .rotate_towards(looking_at.rotation, angle);
            let diff = (rotation - transform.rotation).length();
            if diff > 0.01 {
                transform.rotation = rotation;
            } else {
                let d = speed.value * delta;
                let new_translation = transform.translation.move_towards(destination.value, d);
                transform.translation = new_translation;
            }
        } else {
            commands.trigger_targets(ArrivedEvent, entity);
        }
    }
}

pub fn animate_movement(
    trigger: Trigger<OnAdd, Destination>,
    mut query: Query<&mut AnimationData>,
) {
    if let Ok(mut anim) = query.get_mut(trigger.entity()) {
        anim.animation_index = 1;
    }
}

pub fn idle_on_stop(trigger: Trigger<OnRemove, Destination>, mut query: Query<&mut AnimationData>) {
    if let Ok(mut anim) = query.get_mut(trigger.entity()) {
        anim.animation_index = 0;
    }
}
