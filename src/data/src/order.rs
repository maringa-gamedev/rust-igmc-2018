use super::common::*;
use crate::*;
use log::*;

#[derive(Debug)]
pub enum OrderPossibility {
    OneFlavorNoTopping(FlavorIndex),
    TwoFlavorsNoTopping(FlavorIndex, FlavorIndex),
    ThreeFlavorsNoTopping(FlavorIndex, FlavorIndex, FlavorIndex),
    FourFlavorsNoTopping(FlavorIndex, FlavorIndex, FlavorIndex, FlavorIndex),
    OneFlavorWithTopping(FlavorIndex, ToppingIndex),
    TwoFlavorsWithTopping(FlavorIndex, FlavorIndex, ToppingIndex),
    ThreeFlavorsWithTopping(FlavorIndex, FlavorIndex, FlavorIndex, ToppingIndex),
}

#[derive(Debug)]
pub struct OrderDefinition {
    pub possibility: OrderPossibility,
    pub preparation: PreparationIndex,
    pub timer: f32,
    initial_timer: f32,
}

impl OrderDefinition {
    pub fn new(possibility: OrderPossibility, preparation: PreparationIndex, timer: f32) -> Self {
        OrderDefinition {
            possibility,
            preparation,
            timer,
            initial_timer: timer,
        }
    }

    pub fn matches(&self, order: &Order) -> bool {
        if order.preparation != self.preparation {
            false
        } else {
            match &self.possibility {
                OrderPossibility::OneFlavorNoTopping(ref f1) => {
                    match (
                        &order.flavor_a,
                        &order.flavor_b,
                        &order.flavor_c,
                        &order.flavor_d,
                        &order.topping,
                    ) {
                        (Some(ref a), None, None, None, None) => a == f1,
                        _ => false,
                    }
                }
                OrderPossibility::TwoFlavorsNoTopping(ref f1, ref f2) => {
                    match (
                        &order.flavor_a,
                        &order.flavor_b,
                        &order.flavor_c,
                        &order.flavor_d,
                        &order.topping,
                    ) {
                        (Some(a), Some(b), None, None, None) => {
                            (a == f1 && b == f2) || (a == f2 && b == f1)
                        }
                        _ => false,
                    }
                }
                OrderPossibility::ThreeFlavorsNoTopping(ref f1, ref f2, ref f3) => {
                    match (
                        &order.flavor_a,
                        &order.flavor_b,
                        &order.flavor_c,
                        &order.flavor_d,
                        &order.topping,
                    ) {
                        (Some(a), Some(b), Some(c), None, None) => {
                            (a == f1 && b == f2 && c == f3)
                                || (a == f1 && c == f2 && b == f3)
                                || (b == f1 && a == f2 && c == f3)
                                || (b == f1 && c == f2 && a == f3)
                                || (c == f1 && a == f2 && b == f3)
                                || (c == f1 && b == f2 && a == f3)
                        }
                        _ => false,
                    }
                }
                OrderPossibility::FourFlavorsNoTopping(ref f1, ref f2, ref f3, ref f4) => {
                    match (
                        &order.flavor_a,
                        &order.flavor_b,
                        &order.flavor_c,
                        &order.flavor_d,
                        &order.topping,
                    ) {
                        (Some(a), Some(b), Some(c), Some(d), None) => {
                            (a == f1 && b == f2 && c == f3 && d == f4)
                                || (a == f1 && b == f2 && d == f3 && c == f4)
                                || (a == f1 && c == f2 && b == f3 && d == f4)
                                || (a == f1 && c == f2 && d == f3 && b == f4)
                                || (a == f1 && d == f2 && b == f3 && c == f4)
                                || (a == f1 && d == f2 && c == f3 && b == f4)
                                || (b == f1 && a == f2 && c == f3 && d == f4)
                                || (b == f1 && a == f2 && d == f3 && c == f4)
                                || (b == f1 && c == f2 && a == f3 && d == f4)
                                || (b == f1 && c == f2 && d == f3 && a == f4)
                                || (b == f1 && d == f2 && a == f3 && c == f4)
                                || (b == f1 && d == f2 && c == f3 && a == f4)
                                || (c == f1 && a == f2 && b == f3 && d == f4)
                                || (c == f1 && a == f2 && d == f3 && b == f4)
                                || (c == f1 && b == f2 && a == f3 && d == f4)
                                || (c == f1 && b == f2 && d == f3 && a == f4)
                                || (c == f1 && d == f2 && a == f3 && b == f4)
                                || (c == f1 && d == f2 && b == f3 && a == f4)
                                || (d == f1 && a == f2 && b == f3 && c == f4)
                                || (d == f1 && a == f2 && c == f3 && b == f4)
                                || (d == f1 && b == f2 && a == f3 && c == f4)
                                || (d == f1 && b == f2 && c == f3 && a == f4)
                                || (d == f1 && c == f2 && a == f3 && b == f4)
                                || (d == f1 && c == f2 && b == f3 && a == f4)
                        }
                        _ => false,
                    }
                }
                OrderPossibility::OneFlavorWithTopping(ref f1, ref t1) => {
                    match (
                        &order.flavor_a,
                        &order.flavor_b,
                        &order.flavor_c,
                        &order.flavor_d,
                        &order.topping,
                    ) {
                        (Some(a), None, None, None, Some(t)) => a == f1 && t == t1,
                        _ => false,
                    }
                }
                OrderPossibility::TwoFlavorsWithTopping(ref f1, ref f2, ref t1) => {
                    match (
                        &order.flavor_a,
                        &order.flavor_b,
                        &order.flavor_c,
                        &order.flavor_d,
                        &order.topping,
                    ) {
                        (Some(a), Some(b), None, None, Some(t)) => {
                            (a == f1 && b == f2 && t == t1) || (a == f2 && b == f1 && t == t1)
                        }
                        _ => false,
                    }
                }
                OrderPossibility::ThreeFlavorsWithTopping(ref f1, ref f2, ref f3, ref t1) => {
                    match (
                        &order.flavor_a,
                        &order.flavor_b,
                        &order.flavor_c,
                        &order.flavor_d,
                        &order.topping,
                    ) {
                        (Some(a), Some(b), Some(c), None, Some(t)) => {
                            (a == f1 && b == f2 && c == f3 && t == t1)
                                || (a == f1 && c == f2 && b == f3 && t == t1)
                                || (b == f1 && a == f2 && c == f3 && t == t1)
                                || (b == f1 && c == f2 && a == f3 && t == t1)
                                || (c == f1 && a == f2 && b == f3 && t == t1)
                                || (c == f1 && b == f2 && a == f3 && t == t1)
                        }
                        _ => false,
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Order {
    pub flavor_a: Option<FlavorIndex>,
    pub flavor_b: Option<FlavorIndex>,
    pub flavor_c: Option<FlavorIndex>,
    pub flavor_d: Option<FlavorIndex>,
    pub topping: Option<ToppingIndex>,
    pub preparation: PreparationIndex,
    pub completed: bool,
    pub base_worth: f32,
    pub calc_worth: f32,
    pub delivery_timer: f32,
    delivery_initial: f32,
}

impl Order {
    pub fn new(p: PreparationIndex, f: FlavorIndex) -> Self {
        Order {
            flavor_a: Some(f),
            flavor_b: None,
            flavor_c: None,
            flavor_d: None,
            topping: None,
            preparation: p,
            completed: false,
            base_worth: 100.0,
            calc_worth: 0.0,
            delivery_timer: 32.0,
            delivery_initial: 32.0,
        }
    }

    pub fn calculate_worth(&self) -> isize {
        let percent_delivery_timer = self.delivery_timer / self.delivery_initial;
        let percent_bonus = if percent_delivery_timer >= 0.8 {
            0.15
        } else if percent_delivery_timer >= 0.5 {
            0.1
        } else {
            0.0
        };
        let total = self.base_worth * (1.0 + percent_bonus);
        let total = total.round() as isize;
        total
    }

    pub fn flavor_count(&self) -> usize {
        let mut count = 0;
        if let Some(_) = self.flavor_a {
            count += 1;
        }
        if let Some(_) = self.flavor_b {
            count += 1;
        }
        if let Some(_) = self.flavor_c {
            count += 1;
        }
        if let Some(_) = self.flavor_d {
            count += 1;
        }
        count
    }

    pub fn ingredient_count(&self) -> usize {
        let mut count = 0;
        if let Some(_) = self.flavor_a {
            count += 1;
        }
        if let Some(_) = self.flavor_b {
            count += 1;
        }
        if let Some(_) = self.flavor_c {
            count += 1;
        }
        if let Some(_) = self.flavor_d {
            count += 1;
        }
        if let Some(_) = self.topping {
            count += 1;
        }
        count
    }

    pub fn insert_flavor(&mut self, f: FlavorIndex) {
        if let None = self.flavor_a {
            self.flavor_a = Some(f);
        } else if let None = self.flavor_b {
            self.flavor_b = Some(f);
        } else if let None = self.flavor_c {
            self.flavor_c = Some(f);
        } else if let None = self.flavor_d {
            self.flavor_d = Some(f);
        } else {
            warn!("CANNOT INSERT MORE FLAVORS, SOMETHING MUST BE WRONG!");
        }
    }

    pub fn insert_topping(&mut self, t: ToppingIndex) {
        if let None = self.topping {
            self.topping = Some(t);
        } else {
            warn!("CANNOT INSERT MORE TOPPINGS, SOMETHING MUST BE WRONG!");
        }
    }

    pub fn update_delivery(&mut self, d: f32) {
        self.delivery_timer -= d;
    }

    pub fn has_topping(&self) -> bool {
        self.topping.is_some()
    }

    pub fn has_melted(&self) -> bool {
        self.delivery_timer <= 0.0
    }

    pub fn is_empty(&self) -> bool {
        if let Some(_) = self.flavor_a {
            false
        } else if let Some(_) = self.flavor_b {
            false
        } else if let Some(_) = self.flavor_c {
            false
        } else if let Some(_) = self.flavor_d {
            false
        } else {
            true
        }
    }

    /// This will panic if indexes are not present inside the global definitions resource.
    /// This will generate a panic when searching for a string key in animations for four
    /// flavors and a topping, since we are not producing those assets and keeping generation up
    /// to four ingredients total.
    ///
    pub fn get_flavor_a_key(&self, defs: &Definitions) -> String {
        if let Some(flavor) = &self.flavor_a {
            let flavor = defs
                .flavors()
                .find(|f| f.index == *flavor)
                .unwrap()
                .key
                .clone();
            format!("{}_ball", flavor)
        } else {
            String::from("empty_item")
        }
    }

    pub fn get_flavor_b_key(&self, defs: &Definitions) -> String {
        if let Some(flavor) = &self.flavor_b {
            let flavor = defs
                .flavors()
                .find(|f| f.index == *flavor)
                .unwrap()
                .key
                .clone();
            format!("{}_ball", flavor)
        } else {
            String::from("empty_item")
        }
    }

    pub fn get_flavor_c_key(&self, defs: &Definitions) -> String {
        if let Some(flavor) = &self.flavor_c {
            let flavor = defs
                .flavors()
                .find(|f| f.index == *flavor)
                .unwrap()
                .key
                .clone();
            format!("{}_ball", flavor)
        } else {
            String::from("empty_item")
        }
    }

    pub fn get_flavor_d_key(&self, defs: &Definitions) -> String {
        if let Some(flavor) = &self.flavor_d {
            let flavor = defs
                .flavors()
                .find(|f| f.index == *flavor)
                .unwrap()
                .key
                .clone();
            format!("{}_ball", flavor)
        } else {
            String::from("empty_item")
        }
    }

    pub fn get_topping_key(&self, defs: &Definitions) -> (String, String) {
        if let Some(topping) = &self.topping {
            let preparation = defs
                .preparations()
                .find(|p| p.index == self.preparation)
                .unwrap()
                .key
                .clone();

            let topping = defs
                .toppings()
                .find(|f| f.index == *topping)
                .unwrap()
                .key
                .clone();

            let count = match self.flavor_count() {
                1 => "one",
                2 => "two",
                3 => "three",
                4 => "four",
                _ => std::unreachable!(),
            };

            (
                format!("{}_{}_{}", topping, count, preparation),
                format!("{}_single_{}", topping, preparation),
            )
        } else {
            (String::from("empty_item"), String::from("empty_item"))
        }
    }

    pub fn get_preparation_key(&self, defs: &Definitions) -> String {
        defs.preparations()
            .find(|p| p.index == self.preparation)
            .unwrap()
            .key
            .clone()
    }

    pub fn get_key_orig(&self, defs: &Definitions) -> String {
        let preparation = defs
            .preparations()
            .find(|p| p.index == self.preparation)
            .unwrap()
            .key
            .clone();

        let topping = if let Some(topping) = &self.topping {
            format!(
                "_{}",
                defs.toppings()
                    .find(|t| t.index == *topping)
                    .unwrap()
                    .key
                    .clone()
            )
        } else {
            String::from("")
        };

        let mut flavors = Vec::with_capacity(4);
        if let Some(flavor) = &self.flavor_a {
            flavors.push(
                defs.flavors()
                    .find(|f| f.index == *flavor)
                    .unwrap()
                    .key
                    .clone(),
            );
        }
        if let Some(flavor) = &self.flavor_b {
            flavors.push(
                defs.flavors()
                    .find(|f| f.index == *flavor)
                    .unwrap()
                    .key
                    .clone(),
            );
        }
        if let Some(flavor) = &self.flavor_c {
            flavors.push(
                defs.flavors()
                    .find(|f| f.index == *flavor)
                    .unwrap()
                    .key
                    .clone(),
            );
        }
        if let Some(flavor) = &self.flavor_d {
            flavors.push(
                defs.flavors()
                    .find(|f| f.index == *flavor)
                    .unwrap()
                    .key
                    .clone(),
            );
        }

        flavors.sort_unstable();

        // Attention to the last paramater, topping is optional, so it already includes the `_`
        // when present.
        format!("{}_{}{}", preparation, flavors.join("_"), topping)
    }

    pub fn get_key(&self) -> &str {
        "neapolitan"
    }
}
