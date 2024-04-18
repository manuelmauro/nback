use bevy::{asset::Handle, ecs::system::Resource};
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_kira_audio::AudioSource;

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "embedded://audio/c.ogg")]
    pub c: Handle<AudioSource>,
    #[asset(path = "embedded://audio/h.ogg")]
    pub h: Handle<AudioSource>,
    #[asset(path = "embedded://audio/k.ogg")]
    pub k: Handle<AudioSource>,
    #[asset(path = "embedded://audio/l.ogg")]
    pub l: Handle<AudioSource>,
    #[asset(path = "embedded://audio/q.ogg")]
    pub q: Handle<AudioSource>,
    #[asset(path = "embedded://audio/r.ogg")]
    pub r: Handle<AudioSource>,
    #[asset(path = "embedded://audio/s.ogg")]
    pub s: Handle<AudioSource>,
    #[asset(path = "embedded://audio/t.ogg")]
    pub t: Handle<AudioSource>,
}
