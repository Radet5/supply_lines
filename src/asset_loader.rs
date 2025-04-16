use bevy::prelude::*;

use crate::schedule::StartupSet;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub tree: Handle<Scene>,
    pub fruit: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets.in_set(StartupSet::StartupRoundA));
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    let tree_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("Tree.glb"));
    let fruit_scene = asset_server.load(GltfAssetLabel::Scene(0).from_asset("Fruit.glb"));

    *scene_assets = SceneAssets {
        tree: tree_scene,
        fruit: fruit_scene,
    };
}
