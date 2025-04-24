use avian3d::{math::FRAC_PI_2, prelude::*};
use bevy::{math::vec2, prelude::*};
use rand::Rng;
use vleue_navigator::prelude::*;

use crate::movement::Destination;

#[derive(Component)]
pub struct Obstacle;

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VleueNavigatorPlugin)
            .register_type::<EntityPath>()
            .add_event::<FindPathEvent>()
            .add_event::<NoPathFoundEvent>()
            .add_plugins(NavmeshUpdaterPlugin::<Collider, Obstacle>::default())
            .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
            .add_systems(Startup, setup)
            .add_systems(FixedUpdate, find_path)
            .add_systems(FixedUpdate, traverse_path);
    }
}

fn setup(mut commands: Commands) {
    let obstacle_size = 0.5;

    commands.spawn((
        NavMeshSettings {
            // Define the outer borders of the navmesh.
            fixed: Triangulation::from_outer_edges(&[
                vec2(-25.0, -25.0),
                vec2(25.0, -25.0),
                vec2(25.0, 25.0),
                vec2(-25.0, 25.0),
            ]),
            agent_radius: 0.25,
            simplify: 0.005,
            merge_steps: 0,
            ..default()
        },
        NavMeshUpdateMode::Debounced(1.0),
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_rotation_x(FRAC_PI_2)),
    ));
}

#[derive(Event, Debug)]
pub struct FindPathEvent {
    from_point: Vec3,
    to_point: Vec3,
    entity: Entity,
}

impl FindPathEvent {
    pub fn new(from_point: Vec3, to_point: Vec3, entity: Entity) -> Self {
        Self {
            from_point,
            to_point,
            entity,
        }
    }
}

#[derive(Event, Debug)]
pub struct NoPathFoundEvent;

#[derive(Component, Debug, Reflect)]
pub struct EntityPath {
    path: Vec<Vec3>,
}

impl EntityPath {
    pub fn new(path: Vec<Vec3>) -> Self {
        Self { path }
    }
}

pub fn find_path(
    mut navmeshes: ResMut<Assets<NavMesh>>,
    navmesh: Query<(&ManagedNavMesh, Ref<NavMeshStatus>)>,
    mut find_path_event_reader: EventReader<FindPathEvent>,
    mut commands: Commands,
) {
    let (navmesh, status) = navmesh.single();
    if *status != NavMeshStatus::Built {
        // TODO: probably need a more robust way to handle this
        // println!("NavMesh is not ready yet. {:?}", status);
        return;
    }
    // println!("NavMesh {:?}", status);
    if let Some(navmesh) = navmeshes.get_mut(navmesh) {
        for path_request in find_path_event_reader.read() {
            if let Some(mut path) =
                navmesh.transformed_path(path_request.from_point, path_request.to_point)
            {
                // info!(
                //     "found path from {:?} to {:?}: {:?}",
                //     path_request.from_point, path_request.to_point, path
                // );
                path.path.reverse();
                commands
                    .entity(path_request.entity)
                    .insert(EntityPath::new(path.path));
            } else {
                info!(
                    "no path found from {:?} to {:?}",
                    path_request.from_point, path_request.to_point
                );
                commands.trigger_targets(NoPathFoundEvent, path_request.entity);
            }
        }
    }
}

pub fn traverse_path(
    mut commands: Commands,
    mut query: Query<(&mut EntityPath, Entity), Without<Destination>>,
) {
    for (mut path, entity) in query.iter_mut() {
        if let Some(next) = path.path.pop() {
            commands.entity(entity).insert(Destination::new(next));
        } else {
            commands.entity(entity).remove::<EntityPath>();
        }
    }
}
