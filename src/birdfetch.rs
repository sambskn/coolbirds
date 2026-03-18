use bevy::asset::io::AssetSourceBuilder;
use bevy::asset::io::{
    AssetSourceId,
    memory::{Dir, MemoryAssetReader},
};
use bevy::prelude::*;
use birdgen::BirdGenInputs;
use std::path::Path;

#[derive(Resource)]
struct MemoryDir {
    dir: Dir,
}

pub struct BirdFetchPlugin;
impl Plugin for BirdFetchPlugin {
    fn build(&self, app: &mut App) {
        let memory_dir = MemoryDir {
            dir: Dir::default(),
        };
        let reader = MemoryAssetReader {
            root: memory_dir.dir.clone(),
        };

        app.register_asset_source(
            AssetSourceId::from("memory"),
            AssetSourceBuilder::new(move || Box::new(reader.clone())),
        );

        app.insert_resource(memory_dir).add_observer(on_bird_fetch);
    }
}

fn on_bird_fetch(
    fetch_event: On<BirdFetchEvent>,
    mem_dir: ResMut<MemoryDir>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
) {
    let bird_gltf_fetch = fetch_bird_gltf(fetch_event.0.get_bird_seed_string().as_str());
    match bird_gltf_fetch {
        Ok(gltf_str) => {
            info!("got the str {:?}", gltf_str);
            mem_dir
                .dir
                .insert_asset(Path::new("bird.gltf"), gltf_str.into_bytes());
            commands.spawn(SceneRoot(
                asset_server.load(GltfAssetLabel::Scene(0).from_asset("memory://bird.gltf")),
            ));
        }
        Err(_err_str) => {}
    }
}

#[derive(Event)]
pub struct BirdFetchEvent(pub BirdGenInputs);

const BASE_BIRD_API_PATH: &str = "http://localhost:3030/";

pub fn fetch_bird_gltf(bird_seed: &str) -> Result<String, String> {
    let path = format!("{}bird/{}", BASE_BIRD_API_PATH, bird_seed);
    info!("fetching a bird yo: {}", path);
    let bird_json_req = reqwest::blocking::get(path);
    match bird_json_req {
        Ok(res) => Ok(res.text().unwrap()),
        Err(err) => {
            info!("err getting bird: {:?}", err);
            Err(String::from("Failed to request bird"))
        }
    }
}
