use bevy::prelude::*;

use crate::schedule::StartupSet;

#[derive(Component)]
pub struct Ground;

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground.in_set(StartupSet::StartupRoundA));
    }
}

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(100., 100.))),
        MeshMaterial3d(materials.add(Color::srgb(0.529, 0.922, 0.643))),
        Ground,
    ));
}
