use gfx;
use gfx::Encoder;
// use gfx::CommandBuffer;
use gfx::traits::FactoryExt;
use gfx::Slice;
// use gfx::Resources;
use gfx::Bundle;

use gfx_core::Device;

use sdl2::video::{Window, GLContext};
use sdl2::Sdl;
use gfx_window_sdl::Factory as SDLFactory;
use gfx_core::Factory;
use gfx_window_sdl;
use gfx_device_gl::Resources;
use gfx_device_gl::Device as GLDevice;
use gfx_device_gl::CommandBuffer;
use gfx_core::handle::{RenderTargetView, DepthStencilView};

use cgmath;

use config::VideoConfig;
use physics::B2Point;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;


gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    constant Transform {
        transform: [[f32; 4];4] = "u_Transform",
    }
    
    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::ConstantBuffer<Transform> = "Transform",        
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}

pub type Point = Vertex;
impl Point {
    pub fn from_point_and_color(physics_point: &B2Point, color: Color) -> Point {
        return Point {
            pos: [physics_point.x, physics_point.y],
            color: color,
        };
    }
}

pub type Color = [f32; 3];


const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

// Identity matrix
const TRANSFORM: Transform = Transform {
    transform: [[1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]],
};

pub struct DrawObject {
    vertices: Vec<Point>,
    translation: [f32; 2],
    rotation: f32,
    pub transform: Transform,
    color: Color,
    bundle: Bundle<Resources, pipe::Data<Resources>>,
}

impl DrawObject {
    fn new(vertices: Vec<Point>,
           color: Color,
           bundle: Bundle<Resources, pipe::Data<Resources>>)
           -> DrawObject {
        DrawObject {
            vertices: vertices,
            translation: [0.0, 0.0],
            rotation: 0.0,
            transform: TRANSFORM,
            color: color,
            bundle: bundle,
        }
    }

    // , transform: &b2::Transform
    fn gfx_vertices(&self) -> Vec<Point> {
        self.vertices
            .clone()
        // .into_iter()
        // .map(|v| {
        //     let x = (transform.rot.cos * v.x - transform.rot.sin * v.y) + transform.pos.x;
        //     let y = (transform.rot.sin * v.x + transform.rot.cos * v.y) + transform.pos.y;
        //     Vertex {
        //         pos: [x, y],
        //         color: self.color,
        //     }
        // })
        // .collect()
    }
}

pub struct DrawSystem {
    window: Window,
    glcontext: GLContext,
    device: GLDevice,
    factory: SDLFactory,
    color_view: RenderTargetView<Resources, ColorFormat>,
    depth_view: DepthStencilView<Resources, DepthFormat>,
    encoder: Encoder<Resources, CommandBuffer>,
    resize: bool,
}

impl DrawSystem {
    pub fn new(sdl_context: &Sdl, config: &mut VideoConfig) -> DrawSystem {

        // Initialize video
        let video_subsystem = sdl_context.video().unwrap();

        // let gl_attr = video_subsystem.gl_attr();

        // // Don't use deprecated OpenGL functions
        // gl_attr.set_context_profile(GLProfile::Core);

        // // Set the context into debug mode
        // gl_attr.set_context_flags().debug().set();

        // // Set the OpenGL context version (OpenGL 3.2)
        // gl_attr.set_context_version(3, 2);

        let display_mode = video_subsystem.current_display_mode(0).unwrap();

        config.set_auto_resolution(display_mode.w as u32, display_mode.h as u32);

        let config = config;

        let w = config.x_resolution();
        let h = config.y_resolution();

        if config.auto_resolution() {
            info!("Using current (scaled) resolution {:?}x{:?}", w, h);
        }

        let mut builder = video_subsystem.window("Zoo Spree", w, h);
        if config.fullscreen {
            builder.fullscreen();
        }


        let (mut window, mut glcontext, mut device, mut factory, mut color_view, mut depth_view) =
            gfx_window_sdl::init::<ColorFormat, DepthFormat>(builder)
                .expect("gfx_window_sdl::init failed!");

        let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

        DrawSystem {
            window: window,
            glcontext: glcontext,
            device: device,
            factory: factory,
            color_view: color_view,
            depth_view: depth_view,
            encoder: encoder,
            resize: true,
        }
    }

    pub fn create_draw_object(&mut self, vertices: Vec<Vertex>, vertex_count: usize) -> DrawObject {
        let pso = self.factory
            .create_pipeline_simple(include_bytes!("shader/triangle_150.glslv"),
                                    include_bytes!("shader/triangle_150.glslf"),
                                    pipe::new())
            .unwrap();

        // TODO:
        let vertex_buffer = self.factory
            .create_buffer(vertex_count,
                           gfx::buffer::Role::Vertex,
                           gfx::memory::Usage::Dynamic,
                           gfx::Bind::empty())
            .unwrap();


        let slice = Slice::new_match_vertex_buffer(&vertex_buffer);

        let transform_buffer = self.factory.create_constant_buffer(1);


        let data = pipe::Data {
            vbuf: vertex_buffer,
            transform: transform_buffer,
            out: self.color_view.clone(),
        };



        return DrawObject::new(vertices, [1.0, 2.0, 3.0], Bundle::new(slice, pso, data));
    }

    pub fn resize(&mut self) -> () {
        gfx_window_sdl::update_views(&self.window, &mut self.color_view, &mut self.depth_view);
        self.resize = true;
    }

    pub fn pre_render(&mut self) -> () {
        self.encoder.clear(&self.color_view, CLEAR_COLOR);
    }

    pub fn post_render(&mut self) -> () {
        self.encoder.flush(&mut self.device);
        self.window.gl_swap_window();
        self.device.cleanup();
        self.resize = false;
    }

    pub fn draw(&mut self, object: &mut DrawObject) -> () {
        self.encoder.update_buffer(&object.bundle.data.transform, &[object.transform], 0);
        self.encoder
            .update_buffer(&object.bundle.data.vbuf, &object.gfx_vertices()[..], 0)
            .expect("Failed to update vertex buffer");
        object.bundle.encode(&mut self.encoder)
    }
}
