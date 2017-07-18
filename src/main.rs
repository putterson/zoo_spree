#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate gfx;
extern crate gfx_core;
extern crate gfx_window_sdl;
extern crate gfx_device_gl;
extern crate sdl2;

extern crate gfx_text;

extern crate cgmath;

extern crate stl;

mod config;
mod input;
mod game;
mod draw;
mod physics;

use input::{InputSystem, ControllerState};

use draw::DrawSystem;
use draw::DrawComponent;

use physics::PhysicsObject;
use physics::PhysicsSystem;

use game::minigame::MiniGame;
use game::minigames::sumo::Sumo;
// use game::minigames::triangle_buffers::Triangle;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub fn main() {
    // Initialize logging
    env_logger::init().unwrap();

    // Load settings
    let try_config = config::load();
    let mut config = match try_config {
        Ok(config) => config,
        Err(err) => panic!("{:?}", err),
    };


    let sdl_context = sdl2::init().unwrap();

    // Initialize Draw system
    let mut draw_system = DrawSystem::new(&sdl_context, &mut config.video);

    // Init Physics system
    let mut physics_system = PhysicsSystem::new();

    // Init Input system
    let mut input_system = InputSystem::new(&sdl_context, config.input);


    // The active minigame
    // let mut minigame : Triangle = MiniGame::new();
    let mut minigame : Sumo = MiniGame::new(&mut draw_system, &mut physics_system, & input_system);

    // Event loop
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::ControllerButtonDown { .. } |
                Event::ControllerButtonUp { .. } |
                Event::ControllerAxisMotion { .. } |
                Event::ControllerDeviceAdded { .. } |
                Event::ControllerDeviceRemoved { .. }
                => {
                    input_system.update(event);
                }

                Event::Window { win_event, .. } => {
                    match win_event {
                        WindowEvent::Resized(width, height) => {
                            info!("Window resized {:?}x{:?}", width, height);
                            draw_system.resize();
                        }
                        _ => {}
                    }
                }

                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {
                    input_system.update(event);
                }
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        draw_system.pre_render();

        minigame.step(&mut draw_system, &mut physics_system, &mut input_system);
        minigame.render(&mut draw_system);

        draw_system.post_render();

    }
}

//enum Component {
//    Physics(PhysicsObject),
//    Draw(VertexComponent),
//}