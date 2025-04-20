use std::time::Duration;

use bevy::{prelude::*, scene::SceneInstanceReady, utils::HashMap};

use crate::schedule::StartupSet;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub tree: Handle<Scene>,
    pub fruit: Handle<Scene>,
    pub deer: Handle<Scene>,
    pub stag: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .register_type::<AnimationData>()
            .add_systems(Startup, load_assets.in_set(StartupSet::StartupRoundA))
            .add_systems(FixedUpdate, apply_animation_changes);
    }
}

#[derive(Component, Debug)]
struct AnimationObject {
    animations: Vec<AnimationNodeIndex>,
    graph_handle: Handle<AnimationGraph>,
}

#[derive(Resource)]
pub struct Animations(HashMap<&'static str, AnimationObject>);

#[derive(Component, Debug, Reflect)]
pub struct AnimationData {
    pub animation_key: &'static str,
    pub animation_index: usize,
}

fn load_assets(
    mut commands: Commands,
    mut scene_assets: ResMut<SceneAssets>,
    asset_server: Res<AssetServer>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let tree_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("Tree.glb"));
    let fruit_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("Fruit.glb"));
    let deer_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("animals/Deer.glb"));
    let stag_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("animals/Stag.glb"));

    let (deer_graph, deer_node_indices) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(11).from_asset("animals/Deer.glb")), //idle
        asset_server.load(GltfAssetLabel::Animation(9).from_asset("animals/Deer.glb")),  //walk
        asset_server.load(GltfAssetLabel::Animation(4).from_asset("animals/Deer.glb")),  //run
    ]);
    let deer_graph_handle = graphs.add(deer_graph);

    let (stag_graph, stag_node_indices) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(11).from_asset("animals/Stag.glb")), //idle
        asset_server.load(GltfAssetLabel::Animation(9).from_asset("animals/Stag.glb")),  //walk
        asset_server.load(GltfAssetLabel::Animation(4).from_asset("animals/Stag.glb")),  //run
    ]);
    let stag_graph_handle = graphs.add(stag_graph);

    let animations = HashMap::from([
        (
            "deer",
            AnimationObject {
                animations: deer_node_indices,
                graph_handle: deer_graph_handle,
            },
        ),
        (
            "stag",
            AnimationObject {
                animations: stag_node_indices,
                graph_handle: stag_graph_handle,
            },
        ),
    ]);

    // Keep our animation graph in a Resource so that it can be inserted onto
    // the correct entity once the scene actually loads.
    commands.insert_resource(Animations(animations));

    *scene_assets = SceneAssets {
        tree: tree_scene,
        fruit: fruit_scene,
        deer: deer_scene,
        stag: stag_scene,
    };
}

pub fn asset_load_handle(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    animations: Res<Animations>,
    data: Query<&AnimationData>,
    mut players: Query<&mut AnimationPlayer>,
) {
    // println!("{:?}", trigger);
    for child in children.iter_descendants(trigger.entity()) {
        if let Ok(mut player) = players.get_mut(child) {
            if let Ok(adata) = data.get(trigger.entity()) {
                // println!("yooo, {:?}", adata);
                let ani_set = animations.0.get(adata.animation_key).unwrap();
                // println!("{:?}", ani_set);
                let mut transitions = AnimationTransitions::new();
                transitions
                    .play(
                        &mut player,
                        ani_set.animations[adata.animation_index],
                        Duration::from_millis(250),
                        // Duration::ZERO,
                    )
                    .repeat();
                commands
                    .entity(child)
                    .insert(AnimationGraphHandle(ani_set.graph_handle.clone()))
                    .insert(transitions);
            }
        }
    }
}

fn apply_animation_changes(
    animations: Res<Animations>,
    data_query: Query<(Entity, &AnimationData), Changed<AnimationData>>,
    mut player_query: Query<&mut AnimationPlayer>,
    children: Query<&Children>,
) {
    for (root_ent, anim) in data_query.iter() {
        // find the node that actually has the player
        for child in children.iter_descendants(root_ent) {
            if let Ok(mut player) = player_query.get_mut(child) {
                let obj = &animations.0[anim.animation_key];
                let node = obj.animations[anim.animation_index];

                player.stop_all();

                player.play(node).repeat();
                break;
            }
        }
    }
}
