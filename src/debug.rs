use avian3d::prelude::*;
use bevy::{color::palettes, prelude::*, time::common_conditions::on_timer};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::time::Duration;
use vleue_navigator::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::new());
        // .add_systems(FixedUpdate, print_collisions);
        // .add_plugins(PhysicsDebugPlugin::default())
        // .add_systems(
        //     Update,
        //     view_navmesh.run_if(on_timer(Duration::from_secs_f32(1.0))),
        // );
        // .add_systems(Startup, setup);
    }
}

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     commands.spawn((
//         Mesh3d(meshes.add(Cuboid::default())),
//         MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
//         Transform::from_xyz(0.0, 0.5, 0.0),
//     ));
// }

// fn print_collisions(mut collision_event_reader: EventReader<Collision>) {
//     for Collision(contacts) in collision_event_reader.read() {
//         println!(
//             "Entities {} and {} are colliding {:?}",
//             contacts.entity1,
//             contacts.entity2,
//             contacts.find_deepest_contact(),
//         );
//     }
// }

fn view_navmesh(
    mut commands: Commands,
    navmeshes: Query<Entity, With<ManagedNavMesh>>,
    mut current: Local<usize>,
) {
    for (i, entity) in navmeshes.iter().sort::<Entity>().enumerate() {
        commands.entity(entity).remove::<NavMeshDebug>();
        if i == *current {
            commands
                .entity(entity)
                .insert(NavMeshDebug(palettes::tailwind::RED_800.into()));
        }
    }
    *current = (*current + 1) % navmeshes.iter().len();
}
