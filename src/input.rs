use std::collections::VecDeque;

use sdl2::controller::GameController;
use sdl2::controller::Button;
use sdl2::controller::Axis;
use sdl2::event::Event;
use sdl2::GameControllerSubsystem;
use sdl2::Sdl;

use self::InputEvent::{InputAdded, InputRemoved};

pub enum InputEvent {
    InputAdded(ID),
    InputRemoved(ID),
}

pub struct InputSystem {
    sdl_controller_subsystem: GameControllerSubsystem,
    open_sdl_controllers: Vec<GameController>,
    controller_states: Vec<ControllerState>,
    pub event_queue: VecDeque<InputEvent>,
}

pub type ID = i32;

#[derive(Default)]
#[derive(Debug)]
pub struct ControllerState {
    pub id: u32,
    pub inst_id: ID,
    pub guid: String,

    pub button_x: bool,
    pub button_y: bool,
    pub button_a: bool,
    pub button_b: bool,
    pub button_l_shoulder: bool,
    pub button_r_shoulder: bool,
    pub button_guide: bool,
    pub button_back: bool,
    pub button_start: bool,
    pub button_l_stick: bool,
    pub button_r_stick: bool,
    pub button_up: bool,
    pub button_down: bool,
    pub button_left: bool,
    pub button_right: bool,

    pub axis_l_trigger: i16,
    pub axis_r_trigger: i16,
    pub axis_l_x: i16,
    pub axis_l_y: i16,
    pub axis_r_x: i16,
    pub axis_r_y: i16,
}

impl InputSystem {
    pub fn new(sdl_context: &Sdl) -> InputSystem {
        // Initialize controller
        let controller_subsystem = sdl_context.game_controller().unwrap();

        // Enable controller events
        if !controller_subsystem.event_state() {
            controller_subsystem.set_event_state(true);
        }

        return InputSystem {
            sdl_controller_subsystem: controller_subsystem,
            open_sdl_controllers: vec![],
            controller_states: vec![],
            event_queue: VecDeque::new(),
        }
    }

    pub fn update(&mut self, event: Event) {
        match event {
            Event::ControllerButtonDown { which, button, .. } => {
                for c in self.controller_states.iter_mut() {
                    if c.inst_id == which {
                        c.set_button(button, true);
                    }
                }
            }
            Event::ControllerButtonUp { which, button, .. } => {
                for c in self.controller_states.iter_mut() {
                    if c.inst_id == which {
                        c.set_button(button, false);
                    }
                }
            }
            Event::ControllerAxisMotion { which, axis, value, .. } => {
                for c in self.controller_states.iter_mut() {
                    if c.inst_id == which {
                        c.set_axis(axis, value);
                    }
                }
            }
            Event::ControllerDeviceAdded { which, .. } => {
                info!("Controller {:?} Added", which);
                let controller = self.sdl_controller_subsystem.open(which as u32).unwrap();
                let state = ControllerState::from(&controller);
                let id = state.inst_id;
                self.controller_states.push(state);
                self.open_sdl_controllers.push(controller);

                self.event_queue.push_back(InputAdded(id));

                info!("Open controllers size {:?}", self.open_sdl_controllers.len());
                debug_controllers(&self.open_sdl_controllers);
            }

            Event::ControllerDeviceRemoved { which, .. } => {
                info!("Controller {:?} Removed", which);
                self.open_sdl_controllers.retain(|ref controller| which != controller.instance_id());
                self.controller_states.retain(|ref controller_state| which != controller_state.inst_id);

                self.event_queue.push_back(InputRemoved(which));

                info!("Open controllers size {:?}", self.open_sdl_controllers.len());
                info!("Controller state size {:?}", self.controller_states.len());
                debug_controllers(&self.open_sdl_controllers);
            }
            _ => {}
        }
    }

    pub fn controller_ids(&self) -> Vec<ID> {
        return self.controller_states.iter().map(|c| c.inst_id).collect();
    }

    pub fn get_controller_state(&self, id: ID) -> Option<&ControllerState> {
        return self.controller_states.iter().filter(|c| c.inst_id == id).next();
    }

    pub fn event(&mut self) -> Option<InputEvent> {
        return self.event_queue.pop_front();
    }
}


impl ControllerState {
    fn set_button(&mut self, button: Button, value: bool) {
        match button {
            Button::X => { self.button_x = value }
            Button::Y => { self.button_y = value }
            Button::A => { self.button_a = value }
            Button::B => { self.button_b = value }
            Button::LeftShoulder => { self.button_l_shoulder = value }
            Button::RightShoulder => { self.button_r_shoulder = value }
            Button::Guide => { self.button_guide = value }
            Button::Back => { self.button_back = value }
            Button::Start => { self.button_start = value }
            Button::LeftStick => { self.button_l_stick = value }
            Button::RightStick => { self.button_r_stick = value }
            Button::DPadUp => { self.button_up = value }
            Button::DPadDown => { self.button_down = value }
            Button::DPadLeft => { self.button_left = value }
            Button::DPadRight => { self.button_right = value }
        }
    }

    fn set_axis(&mut self, axis: Axis, value: i16) {
        match axis {
            Axis::TriggerLeft => { self.axis_l_trigger = value }
            Axis::TriggerRight => { self.axis_r_trigger = value }
            Axis::LeftX => { self.axis_l_x = value }
            Axis::LeftY => { self.axis_l_y = value }
            Axis::RightX => { self.axis_r_x = value }
            Axis::RightY => { self.axis_r_y = value }
        }
    }
}

impl<'a> From<&'a GameController> for ControllerState {
    fn from(controller: &'a GameController) -> Self {
        let mut state = ControllerState::default();

        state.inst_id = controller.instance_id();

        state.set_button(Button::X, controller.button(Button::X));
        state.set_button(Button::Y, controller.button(Button::Y));
        state.set_button(Button::A, controller.button(Button::A));
        state.set_button(Button::B, controller.button(Button::B));
        state.set_button(Button::LeftShoulder, controller.button(Button::LeftShoulder));
        state.set_button(Button::RightShoulder, controller.button(Button::RightShoulder));
        state.set_button(Button::Guide, controller.button(Button::Guide));
        state.set_button(Button::Back, controller.button(Button::Back));
        state.set_button(Button::Start, controller.button(Button::Start));
        state.set_button(Button::LeftStick, controller.button(Button::LeftStick));
        state.set_button(Button::RightStick, controller.button(Button::RightStick));
        state.set_button(Button::DPadUp, controller.button(Button::DPadUp));
        state.set_button(Button::DPadDown, controller.button(Button::DPadDown));
        state.set_button(Button::DPadLeft, controller.button(Button::DPadLeft));
        state.set_button(Button::DPadRight, controller.button(Button::DPadRight));

        state.set_axis(Axis::TriggerLeft, controller.axis(Axis::TriggerLeft));
        state.set_axis(Axis::TriggerRight, controller.axis(Axis::TriggerRight));
        state.set_axis(Axis::LeftX, controller.axis(Axis::LeftX));
        state.set_axis(Axis::LeftY, controller.axis(Axis::LeftY));
        state.set_axis(Axis::RightX, controller.axis(Axis::RightX));
        state.set_axis(Axis::RightY, controller.axis(Axis::RightY));

        return state;
    }
}

fn debug_controllers(controllers: &Vec<GameController>) {
    for ref c in controllers {
        debug!("controller {:?}", c.instance_id());
    }
}
