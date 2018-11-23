use amethyst::ecs::prelude::*;
use crate::component::*;
use nk_data::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum InteractionKey {
    North,
    South,
    West,
    East,
}

impl Distribution<InteractionKey> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> InteractionKey {
        match rng.gen_range(0, 4) {
            0 => InteractionKey::North,
            1 => InteractionKey::South,
            2 => InteractionKey::West,
            _ => InteractionKey::East,
        }
    }
}

impl InteractionKey {
    pub fn get_str(&self, style: &Style) -> String {
        match style {
            Style::HalfRight | Style::Full => match self {
                InteractionKey::North => String::from("north_right"),
                InteractionKey::South => String::from("south_right"),
                InteractionKey::West => String::from("west_right"),
                InteractionKey::East => String::from("east_right"),
            },
            Style::HalfLeft => match self {
                InteractionKey::North => String::from("north_left"),
                InteractionKey::South => String::from("south_left"),
                InteractionKey::West => String::from("west_left"),
                InteractionKey::East => String::from("east_left"),
            },
        }
    }
}

// Flavor
#[derive(Debug)]
pub struct FlavorInteraction {
    pub key: InteractionKey,
    length: f32,
    pub progress: f32,
    pub flavor: FlavorIndex,
}

impl FlavorInteraction {
    pub fn new(length: f32, flavor: FlavorIndex) -> Self {
        FlavorInteraction {
            key: rand::random(),
            length,
            progress: 0.0,
            flavor,
        }
    }

    pub fn add_progress(&mut self, delta: f32) {
        self.progress += delta / self.length;
    }

    pub fn remove_progress(&mut self, delta: f32) {
        self.progress -= delta / self.length;
        self.progress = if self.progress < 0.0 {
            0.0
        } else {
            self.progress
        }
    }

    pub fn is_complete(&self) -> bool {
        self.progress >= 1.0
    }
}

impl Component for FlavorInteraction {
    type Storage = DenseVecStorage<Self>;
}

// Preparation
#[derive(Debug)]
pub struct PreparationSequence(
    InteractionKey,
    InteractionKey,
    InteractionKey,
    InteractionKey,
);

impl Distribution<PreparationSequence> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PreparationSequence {
        use self::InteractionKey::*;
        match rng.gen_range(0, 24) {
            0 => PreparationSequence(North, South, West, East),
            1 => PreparationSequence(North, South, East, West),

            2 => PreparationSequence(North, West, East, South),
            3 => PreparationSequence(North, West, South, East),

            4 => PreparationSequence(North, East, South, West),
            5 => PreparationSequence(North, East, West, South),

            6 => PreparationSequence(South, West, East, North),
            7 => PreparationSequence(South, West, North, East),

            8 => PreparationSequence(South, East, North, West),
            9 => PreparationSequence(South, East, West, North),

            10 => PreparationSequence(South, North, East, West),
            11 => PreparationSequence(South, North, West, East),

            12 => PreparationSequence(West, East, South, North),
            13 => PreparationSequence(West, East, North, South),

            14 => PreparationSequence(West, South, East, North),
            15 => PreparationSequence(West, South, North, East),

            16 => PreparationSequence(West, North, South, East),
            17 => PreparationSequence(West, North, East, South),

            18 => PreparationSequence(East, South, West, North),
            19 => PreparationSequence(East, South, North, West),

            20 => PreparationSequence(East, West, South, North),
            21 => PreparationSequence(East, West, North, South),

            22 => PreparationSequence(East, North, West, South),
            _ => PreparationSequence(East, North, South, West),
        }
    }
}

#[derive(Debug)]
pub struct PreparationInteraction {
    pub sequence: Vec<PreparationSequence>,
    current: usize,
    pub progress: f32,
}

impl PreparationInteraction {
    pub fn new(sequence_count: usize) -> Self {
        PreparationInteraction {
            sequence: (0..sequence_count).map(|_| rand::random()).collect(),
            current: 0,
            progress: 0.0,
        }
    }

    pub fn process_key(&mut self, key: InteractionKey) {
        let in_struct = self.current % 4;
        let in_vec = self.current / 4;
        let curr_key = match in_struct {
            0 => &self.sequence[in_vec].0,
            1 => &self.sequence[in_vec].1,
            2 => &self.sequence[in_vec].2,
            3 => &self.sequence[in_vec].3,
            _ => panic!("IMPOSSIBURU, in_struct is % 4"),
        };
        if key == *curr_key {
            self.current += 1;
        } else {
            self.current = in_vec;
        }
        self.progress = (self.current / 4) as f32 / self.sequence.len() as f32;
    }

    pub fn current_key(&self, index: usize, style: &Style) -> String {
        let in_struct = self.current % 4;
        let in_vec = self.current / 4;
        if self.progress >= 1.0 || index < in_struct {
            String::from("success")
        } else {
            match index {
                0 => self.sequence[in_vec].0.get_str(&style),
                1 => self.sequence[in_vec].1.get_str(&style),
                2 => self.sequence[in_vec].2.get_str(&style),
                3 => self.sequence[in_vec].3.get_str(&style),
                _ => panic!("IMPOSSIBURU, in_struct is % 4"),
            }
        }
    }

    pub fn is_complete(&self) -> bool {
        self.progress >= 1.0
    }
}

impl Component for PreparationInteraction {
    type Storage = DenseVecStorage<Self>;
}

// Topping
#[derive(Debug)]
pub struct ToppingPair(InteractionKey, InteractionKey);

impl Distribution<ToppingPair> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ToppingPair {
        use self::InteractionKey::*;
        match rng.gen_range(0, 12) {
            0 => ToppingPair(North, South),
            1 => ToppingPair(North, West),
            2 => ToppingPair(North, East),

            3 => ToppingPair(South, North),
            4 => ToppingPair(South, West),
            5 => ToppingPair(South, East),

            6 => ToppingPair(West, North),
            7 => ToppingPair(West, South),
            8 => ToppingPair(West, East),

            10 => ToppingPair(East, North),
            11 => ToppingPair(East, South),
            _ => ToppingPair(East, West),
        }
    }
}

#[derive(Debug)]
pub struct ToppingInteraction {
    pub pair: ToppingPair,
    pub length: f32,
    pub progress: f32,
}

impl ToppingInteraction {
    pub fn new(length: f32) -> Self {
        ToppingInteraction {
            pair: rand::random(),
            length,
            progress: 0.0,
        }
    }
}

impl Component for ToppingInteraction {
    type Storage = DenseVecStorage<Self>;
}

pub struct StartBarPiece;
pub struct MiddleBarPiece;
pub struct EndBarPiece;
pub struct BarBackground;
pub struct BarForeground;

impl Component for StartBarPiece {
    type Storage = DenseVecStorage<Self>;
}

impl Component for MiddleBarPiece {
    type Storage = DenseVecStorage<Self>;
}

impl Component for EndBarPiece {
    type Storage = DenseVecStorage<Self>;
}

impl Component for BarBackground {
    type Storage = DenseVecStorage<Self>;
}

impl Component for BarForeground {
    type Storage = DenseVecStorage<Self>;
}

pub struct HoldKey;
pub struct SequenceKey(pub usize);
pub struct AlternativeKey(pub bool);

impl Component for HoldKey {
    type Storage = DenseVecStorage<Self>;
}

impl Component for SequenceKey {
    type Storage = DenseVecStorage<Self>;
}

impl Component for AlternativeKey {
    type Storage = DenseVecStorage<Self>;
}
