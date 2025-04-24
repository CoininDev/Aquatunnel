use std::{collections::HashMap, sync::Arc, time::Instant};

use legion::{systems::CommandBuffer, Resources, Schedule, World};
use miniquad::*;
use miniquad::TextureId;
use crate::{entitites::populate, input::InputContext, physics::PhysicsContext, sys::*};

pub struct Game {
    load_sch: Schedule,
    step_sch: Schedule,
    draw_sch: Schedule,
    main_world: World,
    resources: Resources,
}

pub struct Time {
    pub last: Instant,
    pub delta: f32,
}

impl Game {
    pub fn load(&mut self) {
        self.load_sch.execute(&mut self.main_world, &mut self.resources);
    }
}

impl Default for Game {

    fn default() -> Self {
        let mut main_world = World::default();
        populate(&mut main_world);

        let mut resources = Resources::default();
        resources.insert(HashMap::<String, Arc<TextureId>>::new());
        resources.insert(CommandBuffer::new(&main_world));
        resources.insert(Time { last:Instant::now(), delta:0.0 });
        resources.insert(PhysicsContext::default());
        
        let load_sch = Schedule::builder()
            .add_thread_local(load::load_system())
            .add_thread_local(load::load_spritesheet_system())
            .add_system(load::load_physics_system())
            .build();

        let step_sch = Schedule::builder()
            .add_system(tick::time_update_system())
            //.add_thread_local(tick::input_update_system())
            .add_system(tick::step_animation_system(0.0))
            .add_system(tick::step_physics_system())
            .add_system(tick::physics_integration_system())
            .flush()
            .add_thread_local(tick::move_player_system())
            .build();

        let draw_sch = Schedule::builder()
            .add_thread_local(render::clear_screen_system())
            .add_thread_local(render::render_system())
            .add_thread_local(render::draw_fps_system(
                0,
                String::from("0"),
                Instant::now(),
            ))
            .add_thread_local(render::present_system())
            .build();

        
        let mut game = Game { 
            load_sch, 
            step_sch, 
            draw_sch, 
            main_world, 
            resources
        };
        game.load();

        game
    }
}

impl EventHandler for Game {
    fn update(&mut self) {
        self.step_sch.execute(&mut self.main_world, &mut self.resources);
    }

    fn draw(&mut self) {
        self.draw_sch.execute(&mut self.main_world, &mut self.resources);
    }


    fn key_down_event(&mut self, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        self.resources.get_mut::<InputContext>().unwrap().handle_key_down(keycode);
    }

    fn key_up_event(&mut self, keycode: KeyCode, _keymods: KeyMods) {
        self.resources.get_mut::<InputContext>().unwrap().handle_key_up(keycode);
    }

    fn mouse_motion_event(&mut self, x: f32, y: f32) {
        self.resources.get_mut::<InputContext>().unwrap().handle_mouse_motion(x, y);
    }

    fn mouse_button_down_event(&mut self, button: MouseButton, _x: f32, _y: f32) {
        self.resources.get_mut::<InputContext>().unwrap().handle_mouse_button_down(button);
    }

    fn mouse_button_up_event(&mut self, button: MouseButton, _x: f32, _y: f32) {
        self.resources.get_mut::<InputContext>().unwrap().handle_mouse_button_up(button);
    }
}