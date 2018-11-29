use amethyst::{renderer::SpriteSheetHandle, ui::FontHandle};
use serde_derive::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationFrame(pub String, pub f32);
#[derive(Debug, Serialize, Deserialize)]
pub struct AnimationDef(pub Vec<AnimationFrame>, pub AnimationLoop);

#[derive(Debug)]
pub struct Animation {
    pub vec: Vec<(usize, f32, f32, f32)>, // index, duration, end_time, rev_end_time
    pub loop_type: AnimationLoop,
    pub timer: f32,
    handle: SpriteSheetHandle,
    total_time: f32,
    bounce: bool,
    first: bool,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum AnimationLoop {
    Circular,
    Bounce,
    Once,
}

impl Animation {
    pub fn new(
        handle: SpriteSheetHandle,
        vec: Vec<(usize, f32)>,
        loop_type: AnimationLoop,
    ) -> Self {
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
            handle,
            timer: 0.0,
            total_time,
            bounce: false,
            first: true,
        }
    }

    pub fn with_same_frame_step(
        handle: SpriteSheetHandle,
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
            handle,
            timer: 0.0,
            total_time,
            bounce: false,
            first: true,
        }
    }

    pub fn reset(&mut self) {
        self.timer = 0.0;
        self.first = true;
    }

    pub fn update_timer(&mut self, delta: f32) {
        self.timer += delta;
        while self.timer > self.total_time {
            self.timer -= self.total_time;
            self.bounce = !self.bounce;
            self.first = false;
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
                if self.first {
                    self.vec.iter().find(|&&a| self.timer < a.2).unwrap().0
                } else {
                    self.vec.iter().last().unwrap().0
                }
            }
        }
    }

    pub fn get_frame_at(&self, timer: f32, rev: bool) -> usize {
        match rev {
            true => {
                self.vec
                    .iter()
                    .rev()
                    .find(|&&a| timer < a.3)
                    .unwrap_or(self.vec.iter().rev().last().unwrap())
                    .0
            }
            false => {
                self.vec
                    .iter()
                    .find(|&&a| timer < a.2)
                    .unwrap_or(self.vec.iter().last().unwrap())
                    .0
            }
        }
    }

    pub fn obtain_handle(&self) -> SpriteSheetHandle {
        self.handle.clone()
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

pub struct Handles {
    pub player_handle: SpriteSheetHandle,
    pub items_handle: SpriteSheetHandle,
    pub map_handle: SpriteSheetHandle,
    pub bg_handle: SpriteSheetHandle,
    pub empty_handle: SpriteSheetHandle,
    pub hud_handle: SpriteSheetHandle,
    pub buttons_handle: SpriteSheetHandle,
    pub progress_handle: SpriteSheetHandle,
    pub score_font: SpriteSheetHandle,
    pub timer_font: SpriteSheetHandle,
}
