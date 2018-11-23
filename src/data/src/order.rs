use super::common::*;
use log::*;

pub enum OrderPossibility {
    OneFlavourNoTopping(FlavorIndex),
    TwoFlavourNoTopping(FlavorIndex, FlavorIndex),
    ThreeFlavourNoTopping(FlavorIndex, FlavorIndex, FlavorIndex),
    FourFlavourNoTopping(FlavorIndex, FlavorIndex, FlavorIndex, FlavorIndex),
    OneFlavourWithTopping(FlavorIndex, ToppingIndex),
    TwoFlavourWithTopping(FlavorIndex, FlavorIndex, ToppingIndex),
    ThreeFlavourWithTopping(FlavorIndex, FlavorIndex, FlavorIndex, ToppingIndex),
}

pub struct OrderDefinition {
    pub possibility: OrderPossibility,
    pub preparation: PreparationIndex,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub flavor_a: Option<FlavorIndex>,
    pub flavor_b: Option<FlavorIndex>,
    pub flavor_c: Option<FlavorIndex>,
    pub flavor_d: Option<FlavorIndex>,
    pub topping: Option<ToppingIndex>,
    pub preparation: PreparationIndex,
    pub completion: f32,
    pub base_worth: f32,
    pub calc_worth: f32,
    pub delivery_timer: f32,
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
            completion: 0.0,
            base_worth: 0.0,
            calc_worth: 0.0,
            delivery_timer: 32.0,
        }
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

    pub fn is_completed(&self) -> bool {
        self.completion >= 1.0
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

    pub fn update_completion(&mut self, d: f32) {
        self.completion += d;
    }

    pub fn get_key(&self) -> &str {
        "neapolitan"
    }
}
