use amethyst::ecs::prelude::*;
use nk_data::*;

pub enum Action {
    Flavor(FlavorIndex),
    Preparation(PreparationIndex, Option<Order>),
    Topping(ToppingIndex),
    Empty(Option<Order>),
    Delivery,
}

pub struct Table {
    action: Action,
}

impl Table {
    pub fn new_empty_table() -> Self {
        Table {
            action: Action::Empty(None),
        }
    }
    pub fn new_flavor_table(f: FlavorIndex) -> Self {
        Table {
            action: Action::Flavor(f),
        }
    }
    pub fn new_preparation_table(p: PreparationIndex) -> Self {
        Table {
            action: Action::Preparation(p, None),
        }
    }
    pub fn new_topping_table(t: ToppingIndex) -> Self {
        Table {
            action: Action::Topping(t),
        }
    }
    pub fn new_delivery_table() -> Self {
        Table {
            action: Action::Delivery,
        }
    }

    pub fn flavor(&self) -> Option<FlavorIndex> {
        if let Action::Flavor(f) = &self.action {
            Some(f.clone())
        } else {
            None
        }
    }
    pub fn preparation(&self) -> Option<PreparationIndex> {
        if let Action::Preparation(p, _) = &self.action {
            Some(p.clone())
        } else {
            None
        }
    }
    pub fn topping(&self) -> Option<ToppingIndex> {
        if let Action::Topping(t) = &self.action {
            Some(t.clone())
        } else {
            None
        }
    }
    pub fn delivery(&self) -> bool {
        if let Action::Delivery = self.action {
            true
        } else {
            false
        }
    }

    pub fn has_order(&self) -> bool {
        match self.action {
            Action::Preparation(_, Some(_)) | Action::Empty(Some(_)) => true,
            _ => false,
        }
    }

    pub fn extract_order(&mut self) -> Order {
        if let Action::Empty(Some(o)) = &mut self.action {
            o.clone()
        } else if let Action::Preparation(_, Some(o)) = &self.action {
            o.clone()
        } else {
            panic!("CANNOT EXTRACT FROM TABLE WITHOUT ORDER SLOT!");
        }
    }

    pub fn insert_order(&mut self, order: Order) {
        if let Action::Empty(_) = self.action {
            self.action = Action::Empty(Some(order));
        } else if let Action::Preparation(p, _) = &self.action {
            self.action = Action::Preparation(p.clone(), Some(order));
        } else {
            panic!("CANNOT INSERT ORDER WHERE THERE CANNOT BE ANY!");
        }
    }
}

impl Component for Table {
    type Storage = DenseVecStorage<Self>;
}
