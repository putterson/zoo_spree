use sdl2::controller::GameController;
use sdl2::controller::Button;
use sdl2::controller::Axis;
use sdl2::event::Event;

pub struct InputState {
    pub controllers: Vec<ControllerState>
}

#[derive(Default)]
#[derive(Debug)]
pub struct ControllerState {
    pub id: u32,
    pub inst_id: i32,
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

impl InputState {
    pub fn update(&mut self, event: Event) {
        match event {
            Event::ControllerButtonDown { which, button, .. } => {
                for c in self.controllers.iter_mut(){
                    if c.inst_id == which {
                        c.set_button(button, true);
                    }
                }
            },
            Event::ControllerButtonUp { which, button, .. } => {
                for c in self.controllers.iter_mut(){
                    if c.inst_id == which {
                        c.set_button(button, false);
                    }
                }
            },
            Event::ControllerAxisMotion { which, axis, value, .. } => {
                for c in self.controllers.iter_mut(){
                    if c.inst_id == which {
                        c.set_axis(axis, value);
                    }
                }
            }
            _ => {}
        }
    }
}


impl ControllerState {
    fn set_button(&mut self, button: Button, value: bool){
        match button {
            Button::X => { self.button_x = value },
            Button::Y => { self.button_y = value },
            Button::A => { self.button_a = value },
            Button::B => { self.button_b = value },
            Button::LeftShoulder => { self.button_l_shoulder = value },
            Button::RightShoulder => { self.button_r_shoulder = value },
            Button::Guide => { self.button_guide = value },
            Button::Back => { self.button_back = value },
            Button::Start => { self.button_start = value },
            Button::LeftStick => { self.button_l_stick = value },
            Button::RightStick => { self.button_r_stick = value },
            Button::DPadUp => { self.button_up = value },
            Button::DPadDown => { self.button_down = value },
            Button::DPadLeft => { self.button_left = value },
            Button::DPadRight => { self.button_right = value },
        }
    }

    fn set_axis(&mut self, axis: Axis, value: i16){
        match axis {
            Axis::TriggerLeft => { self.axis_l_trigger = value },
            Axis::TriggerRight => { self.axis_r_trigger = value },
            Axis::LeftX => { self.axis_l_x = value },
            Axis::LeftY => { self.axis_l_y = value },
            Axis::RightX => { self.axis_r_x = value },
            Axis::RightY => { self.axis_r_y = value },
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
