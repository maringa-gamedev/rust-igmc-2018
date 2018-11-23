use amethyst::{
    assets::AssetStorage,
    audio::{output::Output, Source, SourceHandle},
};

pub struct Sounds {
    pub pickup_sfx: SourceHandle,
}

pub fn play_pickup(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.pickup_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}
