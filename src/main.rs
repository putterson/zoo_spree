#[macro_use]
extern crate log;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate gfx;
extern crate gfx_core;
extern crate gfx_window_sdl;
extern crate sdl2;

extern crate cgmath;

mod config;
mod game;

use gfx::traits::FactoryExt;
use gfx::Device;

use game::minigame::MiniGame;
use game::minigames::triangle::Triangle;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use gfx_core::format::{DepthStencil, Rgba8};

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

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

    // Initialize video
    let video_subsystem = sdl_context.video().unwrap();


    let display_mode = video_subsystem.current_display_mode(0).unwrap();

    config.video.set_auto_resolution(display_mode.w as u32, display_mode.h as u32);

    let config = config;

    let w = config.video.x_resolution();
    let h = config.video.y_resolution();

    if config.video.auto_resolution() {
        info!("Using current (scaled) resolution {:?}x{:?}", w, h);
    }

    let mut builder = video_subsystem.window("Zoo Spree", w, h);
    if config.video.fullscreen {
        builder.fullscreen();
    }


    let (window, glcontext, mut device, mut factory, color_view, depth_view) =
        gfx_window_sdl::init::<Rgba8, DepthStencil>(builder).expect("gfx_window_sdl::init failed!");

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

    // Initialize controller
    let controller_subsystem = sdl_context.game_controller().unwrap();

    // Enable controller events
    if !controller_subsystem.event_state() {
        controller_subsystem.set_event_state(true);
    }

    let mut open_controllers: Vec<sdl2::controller::GameController> = vec![];

    let num_joysticks = controller_subsystem.num_joysticks().unwrap();
    for id in 0..num_joysticks {
        if controller_subsystem.is_game_controller(id) {
            let controller = controller_subsystem.open(id).unwrap();
            open_controllers.push(controller);
        }
    }

    //
    let mut minigame : Triangle<_> = MiniGame::new(&mut factory, &color_view);

    // Event loop
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::ControllerButtonDown { which, button, .. } => {
                    info!("Controller {:?} Button {:?} down", which, button)
                }
                Event::ControllerAxisMotion { which, axis, value, .. } => {
                    info!("Controller {:?} Axis {:?}: {:?}", which, axis, value)
                }

                Event::ControllerDeviceAdded { which, .. } => info!("Controller {:?} Added", which),
                Event::ControllerDeviceRemoved { which, .. } => {
                    info!("Controller {:?} Removed", which)
                }

                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        encoder.clear(&color_view, CLEAR_COLOR);

        minigame.step();
        minigame.render(&mut encoder);

        encoder.flush(&mut device);
        window.gl_swap_window();
        device.cleanup();
    }
}
