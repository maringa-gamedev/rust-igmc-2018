use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{output::Output, AudioSink, OggFormat, Source, SourceHandle},
    ecs::prelude::World,
};
use nk_data::*;
use std::{iter::Cycle, vec::IntoIter};

/*
 *pub struct Music {
 *    pub music: Cycle<IntoIter<SourceHandle>>,
 *}
 */

fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), (), &world.read_resource())
}

pub fn initialise_audio(world: &mut World) {
    /*
     *use AUDIO_BOUNCE;
     *use AUDIO_MUSIC;
     *use AUDIO_SCORE;
     */

    let sound_effects = {
        let loader = world.read_resource::<Loader>();

        let mut sink = world.write_resource::<AudioSink>();
        //sink.set_volume(0.25); // Music is a bit loud, reduce the volume.

        /*
         *let music = AUDIO_MUSIC
         *    .iter()
         *    .map(|file| load_audio_track(&loader, &world, file))
         *    .collect::<Vec<_>>()
         *    .into_iter()
         *    .cycle();
         *let music = Music { music };
         */

        let sound = Sounds {
            pickup_sfx: load_audio_track(&loader, &world, "sound/pickup.ogg"),
        };

        sound
    };

    // Add sound effects to the world. We have to do this in another scope because
    //     // world won't let us insert new resources as long as `Loader` is borrowed.
    world.add_resource(sound_effects);
    //world.add_resource(music);
}
