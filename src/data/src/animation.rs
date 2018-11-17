use serde_derive::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Animation {
    pub vec: Vec<(usize, f32, f32, f32)>, // index, duration, end_time, rev_end_time
    pub loop_type: AnimationLoop,
    pub timer: f32,
    total_time: f32,
    bounce: bool,
    count: usize,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AnimationLoop {
    Circular,
    Bounce,
    Once,
}

impl Animation {
    pub fn new(vec: Vec<(usize, f32)>, loop_type: AnimationLoop) -> Self {
        let total_time = vec.iter().fold(0.0, |acc, e| acc + e.1);
        let mut acc = 0.0;
        Animation {
            vec: vec
                .iter()
                .map(|(i, d)| {
                    let rev = total_time - acc;
                    acc += d;
                    (*i, *d, acc, rev)
                })
                .collect(),
            loop_type,
            timer: 0.0,
            total_time,
            bounce: false,
            count: 0,
        }
    }

    pub fn with_same_frame_step(
        vec: Vec<usize>,
        loop_type: AnimationLoop,
        frame_step: f32,
    ) -> Self {
        let total_time = vec.len() as f32 * frame_step;
        let mut acc = 0.0;
        Animation {
            vec: vec
                .iter()
                .map(|e| {
                    let rev = total_time - acc;
                    acc += frame_step;
                    (*e, frame_step, acc, rev)
                })
                .collect(),
            loop_type,
            timer: 0.0,
            total_time,
            bounce: false,
            count: 0,
        }
    }

    pub fn reset(&mut self) {
        self.timer = 0.0;
    }

    pub fn update_timer(&mut self, delta: f32) {
        self.timer += delta;
        if self.timer > self.total_time {
            self.timer -= self.total_time;
            self.bounce = !self.bounce;
            self.count += 1;
        }
    }

    pub fn get_frame(&self) -> usize {
        match self.loop_type {
            AnimationLoop::Circular => self.vec.iter().find(|&&a| self.timer < a.2).unwrap().0,
            AnimationLoop::Bounce => {
                if self.bounce {
                    self.vec
                        .iter()
                        .rev()
                        .find(|&&a| self.timer < a.3)
                        .unwrap()
                        .0
                } else {
                    self.vec.iter().find(|&&a| self.timer < a.2).unwrap().0
                }
            }
            AnimationLoop::Once => {
                if self.count > 0 {
                    self.vec.iter().last().unwrap().0
                } else {
                    self.vec.iter().find(|&&a| self.timer < a.2).unwrap().0
                }
            }
        }
    }
}

pub struct Animations {
    pub animations: HashMap<String, Animation>,
}

impl Default for Animations {
    fn default() -> Self {
        Animations {
            animations: HashMap::new(),
        }
    }
}
