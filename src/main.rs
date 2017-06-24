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

extern crate cgmath;

extern crate stl;

mod config;
mod input;
mod game;
mod draw;
mod physics;

use gfx::traits::FactoryExt;
use gfx::Device;

use input::{InputState, ControllerState};

use draw::DrawSystem;
use draw::DrawObject;

use physics::PhysicsObject;
use physics::PhysicsSystem;

use game::minigame::MiniGame;
use game::minigames::sumo::Sumo;
// use game::minigames::triangle_buffers::Triangle;

use sdl2::event::Event;
use sdl2::event::WindowEvent;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use gfx_core::format::{DepthStencil, Rgba8};

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

    // Initialize controller
    let controller_subsystem = sdl_context.game_controller().unwrap();

    // Enable controller events
    if !controller_subsystem.event_state() {
        controller_subsystem.set_event_state(true);
    }

    let mut open_controllers: Vec<sdl2::controller::GameController> = vec![];
    let mut input_state = InputState { controllers: vec![] };


    // The active minigame
    // let mut minigame : Triangle = MiniGame::new();
    let mut minigame : Sumo = MiniGame::new(&mut draw_system, &mut physics_system);

    // Event loop
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::ControllerButtonDown { .. } |
                Event::ControllerButtonUp { .. } |
                Event::ControllerAxisMotion { .. } => {
                    input_state.update(event);
                }
                Event::ControllerDeviceAdded { which, .. } => {
                    info!("Controller {:?} Added", which);
                    let controller = controller_subsystem.open(which as u32).unwrap();
                    input_state.controllers.push(ControllerState::from(&controller));
                    open_controllers.push(controller);

                    
                    info!("Open controllers size {:?}", open_controllers.len());
                    debug_controllers(&open_controllers);
                }
                
                Event::ControllerDeviceRemoved { which, .. } => {
                    info!("Controller {:?} Removed", which);
                    open_controllers.retain(|ref controller| which != controller.instance_id());
                    input_state.controllers.retain(|ref controller_state| which != controller_state.inst_id );
                    info!("Open controllers size {:?}", open_controllers.len());
                    info!("Controller state size {:?}", input_state.controllers.len());
                    debug_controllers(&open_controllers);
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
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        draw_system.pre_render();

        minigame.step(&input_state, &mut physics_system);
        minigame.render(&mut draw_system);

        draw_system.post_render();

    }
}

fn debug_controllers(controllers: &Vec<sdl2::controller::GameController>) {
    for ref c in controllers {
        debug!("controller {:?}", c.instance_id());
    }
}

struct Components {
    draw: Option<DrawObject>,
    physics: Option<PhysicsObject>
}