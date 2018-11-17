use amethyst::{ecs::prelude::*, utils::application_root_dir};
use log::*;
use nk_data::*;
use ron::de::from_reader;
use std::fs::File;

pub fn load_game_data(world: &mut World) -> Definitions {
    let app_root = application_root_dir();

    let flavors = {
        let path = format!("{}/assets/data/flavors.ron", app_root);
        let f = File::open(&path).expect("Failed opening 'flavors.ron' file!");
        match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                error!("Error parsing flavors definition: {}", e);
                panic!("Invalid flavors definition <{}>!", path);
            }
        }
    };

    let preparations = {
        let path = format!("{}/assets/data/preparations.ron", app_root);
        let f = File::open(&path).expect("Failed opening 'preparations.ron' file!");
        match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                error!("Error parsing preparations definition: {}", e);
                panic!("Invalid preparations definition <{}>!", path);
            }
        }
    };

    let toppings = {
        let path = format!("{}/assets/data/toppings.ron", app_root);
        let f = File::open(&path).expect("Failed opening 'toppings.ron' file!");
        match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                error!("Error parsing toppings definition: {}", e);
                panic!("Invalid toppings definition <{}>!", path);
            }
        }
    };

    Definitions::new(flavors, preparations, toppings)
}
