use std::collections::{HashMap, VecDeque};
use glam::Vec2;
use miniquad::{KeyCode, MouseButton};

pub struct InputContext {
    pub move_direction: Vec2,
    pub look_direction: Vec2,
    pub actions: VecDeque<InputAction>,
    setup: InputSetup,
    raw_keys: VecDeque<KeyCode>,
    raw_mouse_motion: Vec2,
    raw_mouse_position: Vec2,
}

impl InputContext {
    pub fn new(setup: InputSetup) -> Self {
        InputContext {
            move_direction: Vec2::ZERO,
            look_direction: Vec2::ZERO,
            actions: VecDeque::new(),
            setup,
            raw_keys: VecDeque::new(),
            raw_mouse_motion: Vec2::ZERO,
            raw_mouse_position: Vec2::ZERO,
        }
    }



    pub fn handle_key_down(&mut self, keycode: KeyCode) {
        if let Some(action) = self.setup.keybindings.get(&RawAction::KeyDown(keycode)) {
            self.actions.push_back(*action);
        }
    }

    pub fn handle_key_up(&mut self, keycode: KeyCode) {
        if let Some(action) = self.setup.keybindings.get(&RawAction::KeyUp(keycode)) {
            self.actions.push_back(*action);
        }
    }

    pub fn handle_mouse_motion(&mut self, delta_x: f32, delta_y: f32) {
        self.raw_mouse_motion = Vec2::new(delta_x, delta_y);
        self.raw_mouse_position += Vec2::new(delta_x, delta_y);
    }

    pub fn handle_mouse_button_down(&mut self, button: MouseButton) {
        if let Some(action) = self.setup.keybindings.get(&RawAction::MouseDown(button)){
            self.actions.push_back(*action);
        }
    }

    pub fn handle_mouse_button_up(&mut self, button: MouseButton) {
        if let Some(action) = self.setup.keybindings.get(&RawAction::MouseUp(button)){
            self.actions.push_back(*action);
        }
    }

    // fn method_0(&self) -> Vec2 {
    //     let event_pump_ref = self.event_pump.borrow_mut();
    //     let keyboard_state = event_pump_ref.keyboard_state();
    //     // -A +D
    //     let x_signal = -(keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::A) as i32)
    //         + (keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::D) as i32);
    //     // -W +S
    //     let y_signal = -(keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::W) as i32)
    //         + (keyboard_state.is_scancode_pressed(sdl2::keyboard::Scancode::S) as i32);

    //     Vec2::new(x_signal as f32, y_signal as f32).normalize_or_zero()
    // }
}

#[derive(Clone, Copy)]
pub enum InputAction {
    DebugAction,
    Click(f32,f32),
}

#[derive(PartialEq, Eq, Hash)]
pub enum RawAction {
    KeyDown(KeyCode),
    KeyUp(KeyCode),
    MouseDown(MouseButton),
    MouseUp(MouseButton),
}

pub struct InputSetup {
    keybindings: HashMap<RawAction, InputAction>,
    move_method: u8,
    look_method: u8,
}

impl Default for InputSetup {
    fn default() -> Self {
        let mut keybindings = HashMap::new();
        keybindings.insert(RawAction::KeyDown(KeyCode::D), InputAction::DebugAction);

        InputSetup {
            keybindings,
            move_method: 0,
            look_method: 1,
        }
    }
}
