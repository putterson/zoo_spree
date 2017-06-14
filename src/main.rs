#[macro_use]
extern crate gfx;

extern crate gfx_core;
extern crate gfx_window_sdl;
extern crate sdl2;

use gfx::traits::FactoryExt;
use gfx::Device;

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

const TRIANGLE: [Vertex; 3] = [Vertex {
                                   pos: [-0.5, -0.5],
                                   color: [1.0, 0.0, 0.0],
                               },
                               Vertex {
                                   pos: [0.5, -0.5],
                                   color: [0.0, 1.0, 0.0],
                               },
                               Vertex {
                                   pos: [0.0, 0.5],
                                   color: [0.0, 0.0, 1.0],
                               }];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use gfx_core::format::{DepthStencil, Rgba8};

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut builder = video_subsystem.window("ZooSpree", 800, 600);

    let (window, glcontext,mut device,mut factory, color_view, depth_view) =
        gfx_window_sdl::init::<Rgba8, DepthStencil>(builder).expect("gfx_window_sdl::init failed!");

    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let pso = factory.create_pipeline_simple(include_bytes!("shader/triangle_120.glslv"),
                                include_bytes!("shader/triangle_120.glslf"),
                                pipe::new())
        .unwrap();
    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
    let mut data = pipe::Data {
        vbuf: vertex_buffer,
        out: color_view,
    };


    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        encoder.clear(&data.out, CLEAR_COLOR);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.gl_swap_window();
        device.cleanup();
    }
}
