use amethyst::{
    core::timing::Time,
    ecs::prelude::{Read, System, Write},
};
use log::*;
use nk_data::*;
use rand::{seq::SliceRandom, Rng};

pub struct GenerateSystem;

const STEP: f32 = 20.0;

impl<'s> System<'s> for GenerateSystem {
    type SystemData = (Read<'s, Definitions>, Write<'s, Match>, Read<'s, Time>);

    fn run(&mut self, (defs, mut match_data, time): Self::SystemData) {
        let mut rng = rand::thread_rng();

        match_data.order_gen_timer += time.delta_seconds();
        while match_data.order_gen_timer >= STEP {
            let g_flavors = match_data.flavors.to_owned();
            let g_preparations = match_data.preparations.to_owned();
            let g_toppings = match_data.toppings.to_owned();

            for team in match_data.teams.iter_mut() {
                let mut flavors = team.flavors.to_owned();
                flavors.append(&mut g_flavors.to_owned());
                flavors.dedup();

                let mut preparations = team.preparations.to_owned();
                preparations.append(&mut g_preparations.to_owned());
                preparations.dedup();
                preparations.shuffle(&mut rng);

                let mut toppings = team.toppings.to_owned();
                toppings.append(&mut g_toppings.to_owned());
                toppings.dedup();
                toppings.shuffle(&mut rng);

                if preparations.len() < 1 {
                    panic!("NO PREPARATIONS IN MATCH+TEAM!");
                };

                if toppings.len() < 1 {
                    panic!("NO TOPPINGS IN MATCH+TEAM!");
                };

                if flavors.len() < 1 {
                    panic!("NO FLAVORS IN MATCH+TEAM!");
                };
                let preparation = &defs
                    .preparations()
                    .find(|x| x.index == preparations[0])
                    .unwrap();

                let rand = match preparation.max_scoops {
                    1 => match preparation.takes_topping {
                        true => match rng.gen_range(0, 2) {
                            0 => 0,
                            1 => 4,
                            _ => std::unreachable!(),
                        },
                        false => 0,
                    },
                    2 => match preparation.takes_topping {
                        true => match rng.gen_range(0, 4) {
                            0 => 0,
                            1 => 1,
                            2 => 4,
                            3 => 5,
                            _ => std::unreachable!(),
                        },
                        false => match rng.gen_range(0, 2) {
                            0 => 0,
                            1 => 1,
                            _ => std::unreachable!(),
                        },
                    },
                    3 => match preparation.takes_topping {
                        true => match rng.gen_range(0, 6) {
                            0 => 0,
                            1 => 1,
                            2 => 2,
                            3 => 4,
                            4 => 5,
                            5 => 6,
                            _ => std::unreachable!(),
                        },
                        false => match rng.gen_range(0, 3) {
                            0 => 0,
                            1 => 1,
                            2 => 2,
                            _ => std::unreachable!(),
                        },
                    },
                    4 => match preparation.takes_topping {
                        true => rng.gen_range(0, 7),
                        false => rng.gen_range(0, 4),
                    },
                    _ => std::unreachable!(),
                };

                // 0 => 1f 0t
                // 1 => 2f 0t
                // 2 => 3f 0t
                // 3 => 4f 0t
                // 4 => 1f 1t
                // 5 => 2f 1t
                // 6 => 3f 1t
                let new_order = match rand {
                    0 => {
                        let mut selection = flavors
                            .into_iter()
                            .map(|f| (defs.flavors().find(|e| e.index == f).unwrap().weights[0], f))
                            .collect::<Vec<(usize, FlavorIndex)>>();

                        let f1 = {
                            let my_selection = selection.to_owned();
                            let key = my_selection
                                .choose_weighted(&mut rng, |item| item.0)
                                .unwrap();
                            selection.remove_item(&key).unwrap()
                        };

                        OrderPossibility::OneFlavorNoTopping(f1.1.clone())
                    }
                    1 => {
                        let mut selection = flavors
                            .into_iter()
                            .map(|f| (defs.flavors().find(|e| e.index == f).unwrap().weights[1], f))
                            .collect::<Vec<(usize, FlavorIndex)>>();

                        let f1 = {
                            let my_selection = selection.to_owned();
                            let key = my_selection
                                .choose_weighted(&mut rng, |item| item.0)
                                .unwrap();
                            selection.remove_item(&key).unwrap()
                        };

                        let f2 = {
                            let my_selection = selection.to_owned();
                            let key = my_selection
                                .choose_weighted(&mut rng, |item| item.0)
                                .unwrap();
                            selection.remove_item(&key).unwrap()
                        };

                        OrderPossibility::TwoFlavorsNoTopping(f1.1.clone(), f2.1.clone())
                    }
                    2 => {
                        let mut selection = flavors
                            .into_iter()
                            .map(|f| (defs.flavors().find(|e| e.index == f).unwrap().weights[2], f))
                            .collect::<Vec<(usize, FlavorIndex)>>();

                        let f1 = {
                            let my_selection = selection.to_owned();
                            let key = my_selection
                                .choose_weighted(&mut rng, |item| item.0)
                                .unwrap();
                            selection.remove_item(&key).unwrap()
                        };

                        let f2 = {
                            let my_selection = selection.to_owned();
                            let key = my_selection
                                .choose_weighted(&mut rng, |item| item.0)
                                .unwrap();
                            selection.remove_item(&key).unwrap()
                        };

                        let f3 = {
                            let my_selection = selection.to_owned();
                            let key = my_selection
                                .choose_weighted(&mut rng, |item| item.0)
                                .unwrap();
                            selection.remove_item(&key).unwrap()
                        };

                        OrderPossibility::ThreeFlavorsNoTopping(
                            f1.1.clone(),
                            f2.1.clone(),
                            f3.1.clone(),
                        )
                    }
                    3 => {
                        let mut selection = flavors
                            .into_iter()
                            .map(|f| (defs.flavors().find(|e| e.index == f).unwrap().weights[3], f))
                            .collect::<Vec<(usize, FlavorIndex)>>();

                        let f1 = {
                            let my_selection = selection.to_owned();
                            let key = my_selection
                                .choose_weighted(&mut rng, |item| item.0)
                                .unwrap();
                            selection.remove_item(&key).unwrap()
                        };

                        let f2 = {
                            let my_selection = selection.to_owned();
                            let key = my_selection
                                .choose_weighted(&mut rng, |item| item.0)
                                .unwrap();
                            selection.remove_item(&key).unwrap()
                        };

                        let f3 = {
                            let my_selection = selection.to_owned();
                            let key = my_selection
                                .choose_weighted(&mut rng, |item| item.0)
                                .unwrap();
                            selection.remove_item(&key).unwrap()
                        };

                        let f4 = {
                            let my_selection = selection.to_owned();
                            let key = my_selection
                                .choose_weighted(&mut rng, |item| item.0)
                                .unwrap();
                            selection.remove_item(&key).unwrap()
                        };

                        OrderPossibility::FourFlavorsNoTopping(
                            f1.1.clone(),
                            f2.1.clone(),
                            f3.1.clone(),
                            f4.1.clone(),
                        )
                    }
                    4 => {
                        let mut selection = flavors
                            .into_iter()
                            .map(|f| (defs.flavors().find(|e| e.index == f).unwrap().weights[4], f))
                            .collect::<Vec<(usize, FlavorIndex)>>();

                        let f1 = {
                            let my_selection = selection.to_owned();
                            let key = my_selection
                                .choose_weighted(&mut rng, |item| item.0)
                                .unwrap();
                            selection.remove_item(&key).unwrap()
                        };

                        OrderPossibility::OneFlavorWithTopping(f1.1.clone(), toppings[0].clone())
                    }
                    5 => {
                        let mut selection = flavors
                            .into_iter()
                            .map(|f| (defs.flavors().find(|e| e.index == f).unwrap().weights[5], f))
                            .collect::<Vec<(usize, FlavorIndex)>>();

                        let f1 = {
                            let my_selection = selection.to_owned();
                            let key = my_selection
                                .choose_weighted(&mut rng, |item| item.0)
                                .unwrap();
                            selection.remove_item(&key).unwrap()
                        };

                        let f2 = {
                            let my_selection = selection.to_owned();
                            let key = my_selection
                                .choose_weighted(&mut rng, |item| item.0)
                                .unwrap();
                            selection.remove_item(&key).unwrap()
                        };

                        OrderPossibility::TwoFlavorsWithTopping(
                            f1.1.clone(),
                            f2.1.clone(),
                            toppings[0].clone(),
                        )
                    }
                    6 => {
                        let mut selection = flavors
                            .into_iter()
                            .map(|f| (defs.flavors().find(|e| e.index == f).unwrap().weights[6], f))
                            .collect::<Vec<(usize, FlavorIndex)>>();

                        let f1 = {
                            let my_selection = selection.to_owned();
                            let key = my_selection
                                .choose_weighted(&mut rng, |item| item.0)
                                .unwrap();
                            selection.remove_item(&key).unwrap()
                        };

                        let f2 = {
                            let my_selection = selection.to_owned();
                            let key = my_selection
                                .choose_weighted(&mut rng, |item| item.0)
                                .unwrap();
                            selection.remove_item(&key).unwrap()
                        };

                        let f3 = {
                            let my_selection = selection.to_owned();
                            let key = my_selection
                                .choose_weighted(&mut rng, |item| item.0)
                                .unwrap();
                            selection.remove_item(&key).unwrap()
                        };

                        OrderPossibility::ThreeFlavorsWithTopping(
                            f1.1.clone(),
                            f2.1.clone(),
                            f3.1.clone(),
                            toppings[0].clone(),
                        )
                    }
                    _ => std::unreachable!(),
                };

                info!("gen new order: {:?}", new_order);

                team.orders.push(OrderDefinition::new(
                    new_order,
                    preparations[0].clone(),
                    45.0,
                ));
            }

            match_data.order_gen_timer -= STEP;
        }
    }
}
