use amethyst::{
    core::cgmath::*,
    ecs::prelude::*,
    shrev::{EventChannel, ReaderId},
};
use gilrs::*;
use std::{collections::HashMap, sync::*};

pub struct Controller {
    pub id: usize,
    pub active: bool,
    pub left_axis: Vector3<f32>,
    pub right_axis: Vector3<f32>,
    pub d_pad: Vector2<f32>,
    pub actions: [bool; 4],
    pub select: bool,
    pub start: bool,
    pub thumbs: [bool; 2],
    pub shoulders: [bool; 2],
    pub triggers: [bool; 2],
}

impl Controller {
    pub fn new(id: usize) -> Self {
        Controller {
            id,
            active: false,
            left_axis: Vector3::new(0.0, 0.0, 0.0),
            right_axis: Vector3::new(0.0, 0.0, 0.0),
            d_pad: Vector2::new(0.0, 0.0),
            actions: [false, false, false, false],
            select: false,
            start: false,
            thumbs: [false, false],
            shoulders: [false, false],
            triggers: [false, false],
        }
    }
}

pub struct ControllerSystem {
    reader: Option<ReaderId<ev::Event>>,
}

impl ControllerSystem {
    pub fn new() -> Self {
        ControllerSystem { reader: None }
    }
}

impl<'s> System<'s> for ControllerSystem {
    type SystemData = (
        Read<'s, Arc<Mutex<EventChannel<ev::Event>>>>,
        Read<'s, Arc<Mutex<HashMap<usize, Controller>>>>,
    );

    fn setup(&mut self, mut res: &mut Resources) {
        Self::SystemData::setup(&mut res);
        self.reader = Some(
            res.fetch_mut::<Arc<Mutex<EventChannel<ev::Event>>>>()
                .lock()
                .unwrap()
                .register_reader(),
        );
    }

    fn run(&mut self, (events, controllers): Self::SystemData) {
        if let Some(ref mut reader) = &mut self.reader {
            for Event {
                id,
                event,
                time: _time,
            } in events.lock().unwrap().read(reader)
            {
                let mut controllers = controllers.lock().unwrap();
                match event {
                    ev::EventType::ButtonPressed(b, _) => {
                        if let Some(controller) = controllers.get_mut(id) {
                            match b {
                                ev::Button::South => controller.actions[0] = true,
                                ev::Button::East => controller.actions[1] = true,
                                ev::Button::North => controller.actions[2] = true,
                                ev::Button::West => controller.actions[3] = true,
                                ev::Button::Select => controller.select = true,
                                ev::Button::Start => controller.start = true,
                                ev::Button::LeftThumb => controller.thumbs[0] = true,
                                ev::Button::RightThumb => controller.thumbs[1] = true,
                                ev::Button::LeftTrigger => controller.shoulders[0] = true,
                                ev::Button::RightTrigger => controller.shoulders[1] = true,
                                ev::Button::LeftTrigger2 => controller.triggers[0] = true,
                                ev::Button::RightTrigger2 => controller.triggers[1] = true,
                                ev::Button::DPadUp => controller.d_pad.y = 1.0,
                                ev::Button::DPadDown => controller.d_pad.y = -1.0,
                                ev::Button::DPadLeft => controller.d_pad.x = -1.0,
                                ev::Button::DPadRight => controller.d_pad.x = 1.0,
                                _ => {}
                            }
                        }
                    }
                    ev::EventType::ButtonReleased(b, _) => {
                        if let Some(controller) = controllers.get_mut(id) {
                            match b {
                                ev::Button::South => controller.actions[0] = false,
                                ev::Button::East => controller.actions[1] = false,
                                ev::Button::North => controller.actions[2] = false,
                                ev::Button::West => controller.actions[3] = false,
                                ev::Button::Select => controller.select = false,
                                ev::Button::Start => controller.start = false,
                                ev::Button::LeftThumb => controller.thumbs[0] = false,
                                ev::Button::RightThumb => controller.thumbs[1] = false,
                                ev::Button::LeftTrigger => controller.shoulders[0] = false,
                                ev::Button::RightTrigger => controller.shoulders[1] = false,
                                ev::Button::LeftTrigger2 => controller.triggers[0] = false,
                                ev::Button::RightTrigger2 => controller.triggers[1] = false,
                                ev::Button::DPadUp => controller.d_pad[1] = 0.0,
                                ev::Button::DPadDown => controller.d_pad[1] = 0.0,
                                ev::Button::DPadLeft => controller.d_pad[0] = 0.0,
                                ev::Button::DPadRight => controller.d_pad[0] = 0.0,
                                _ => {}
                            }
                        }
                    }
                    ev::EventType::ButtonChanged(b, v, _) => {
                        if let Some(controller) = controllers.get_mut(id) {
                            match b {
                                ev::Button::DPadUp => controller.d_pad.y = *v,
                                ev::Button::DPadDown => controller.d_pad.y = *v,
                                ev::Button::DPadLeft => controller.d_pad.x = *v,
                                ev::Button::DPadRight => controller.d_pad.x = *v,
                                _ => {}
                            }
                        }
                    }
                    ev::EventType::AxisChanged(a, v, _) => {
                        if let Some(controller) = controllers.get_mut(id) {
                            match a {
                                ev::Axis::LeftStickX => controller.left_axis[0] = *v,
                                ev::Axis::LeftStickY => controller.left_axis[1] = *v,
                                ev::Axis::LeftZ => controller.left_axis[2] = *v,
                                ev::Axis::RightStickX => controller.right_axis[0] = *v,
                                ev::Axis::RightStickY => controller.right_axis[1] = *v,
                                ev::Axis::RightZ => controller.right_axis[2] = *v,
                                ev::Axis::DPadX => controller.d_pad[0] = *v,
                                ev::Axis::DPadY => controller.d_pad[1] = *v,
                                _ => {}
                            }
                        }
                    }
                    ev::EventType::Connected => {
                        let entry = controllers.entry(*id).or_insert(Controller::new(*id));
                        entry.active = true;
                    }
                    ev::EventType::Disconnected => {
                        let entry = controllers.entry(*id).or_insert(Controller::new(*id));
                        entry.active = false;
                    }
                    _ => {}
                }
            }
        }
    }
}
