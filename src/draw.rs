use gfx;
use gfx::Encoder;

use gfx::traits::FactoryExt;
use gfx::Slice;

use gfx::Bundle;

use gfx_core::Device;
use gfx_core::Factory;

use sdl2::video::{Window, GLContext};
use sdl2::Sdl;
use sdl2::video::GLProfile;

use gfx_window_sdl::Factory as SDLFactory;
use gfx_window_sdl;
use gfx_device_gl::Resources;
use gfx_device_gl::Device as GLDevice;
use gfx_device_gl::CommandBuffer;
use gfx_core::handle::{RenderTargetView, DepthStencilView};

use gfx_text::{HorizontalAnchor, VerticalAnchor, Renderer};
use gfx_text;

use stl;
use stl::Triangle;

use game::minigame::Point as WorldPoint;
use config::VideoConfig;
use physics::B2Point;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;


gfx_defines! {
    vertex Vertex {
        pos: [f32; 3] = "a_Pos",
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
    pub fn from_point_and_color(world_point: &WorldPoint, color: Color) -> Point {
        return Point {
            pos: world_point.clone(),
            color: color,
        };
    }

    pub fn from_stl(triangle: &Triangle, color: Color) -> Vec<Point> {
        return vec![
            Point {
                pos: [triangle.v1[0], triangle.v1[1], triangle.v1[2]],
                color: color,
            },
            Point {
                pos: [triangle.v2[0], triangle.v2[1], triangle.v2[2]],
                color: color,
            },
            Point {
                pos: [triangle.v3[0], triangle.v3[1], triangle.v3[2]],
                color: color,
            },
        ];
    }
}

pub type Color = [f32; 3];


const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

// Identity matrix
pub const IDENTITY: Transform = Transform {
    transform: [[1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]],
};

pub trait DrawComponent {
    fn set_color(&mut self, new_color: Color);
    fn draw(&mut self, resize: bool, encoder : &mut Encoder<Resources, CommandBuffer>, color_view: &RenderTargetView<Resources, ColorFormat>);
}

pub struct VertexComponent {
    vertices: Vec<Point>,
    translation: [f32; 2],
    rotation: f32,
    pub transform: Transform,
    bundle: Bundle<Resources, pipe::Data<Resources>>,
    update_model: bool,
}

impl DrawComponent for VertexComponent {
    fn set_color(&mut self, new_color: Color) {
        for vertex in self.vertices.iter_mut() {
            vertex.color = new_color;
        }

        self.update_model = true;
    }

    fn draw(&mut self, resize : bool, encoder: &mut Encoder<Resources, CommandBuffer>, color_view: &RenderTargetView<Resources, ColorFormat>) {
        if resize {
            self.bundle.data.out = color_view.clone();
        }

        encoder.update_constant_buffer(&self.bundle.data.transform, &self.transform);

        if self.update_model {
            encoder
                .update_buffer(&self.bundle.data.vbuf, &self.vertices.clone()[..], 0)
                .expect("Failed to update vertex buffer");
            self.update_model = false;
        }

        self.bundle.encode(encoder)
    }
}


pub struct TextComponent {
    pub color: Color,
    pub text: String,
    pub transform: Transform,
    renderer: Renderer<Resources, SDLFactory>,
}

impl DrawComponent for TextComponent {
    fn set_color(&mut self, new_color: Color) {
        self.color = new_color;
    }
    fn draw(&mut self, resize: bool, encoder: &mut Encoder<Resources, CommandBuffer>, color_view: &RenderTargetView<Resources, ColorFormat>) {
        self.renderer.add_at(
            self.text.as_ref(),
            [-self.transform.transform[3][0], -self.transform.transform[3][1], 1.0],
            [self.color[0], self.color[1], self.color[2], 1.0]);
        self.renderer.draw_at(encoder, color_view, IDENTITY.transform).unwrap();
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

        let gl_attr = video_subsystem.gl_attr();

        // // Don't use deprecated OpenGL functions
        gl_attr.set_context_profile(GLProfile::Core);

        // // Set the context into debug mode
        // gl_attr.set_context_flags().debug().set();

        // // Set the OpenGL context version (OpenGL 3.3)
        gl_attr.set_context_version(3, 3);

        let display_mode = video_subsystem.current_display_mode(0).unwrap();

        config.set_auto_resolution(display_mode.w as u32, display_mode.h as u32);

        let config = config;

        let w = config.x_resolution();
        let h = config.y_resolution();

        if config.auto_resolution() {
            info! ("Using current (scaled) resolution {:?}x{:?}", w, h);
        }

        let mut builder = video_subsystem.window("Zoo Spree", w, h);
        if config.fullscreen {
            builder.fullscreen();
        }


        let (window, glcontext, device, mut factory, color_view, depth_view) =
            gfx_window_sdl::init::<ColorFormat, DepthFormat>(builder)
                .expect("gfx_window_sdl::init failed!");

        let encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();

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

    pub fn new_vertex_component(vertices: Vec<Point>,
                                bundle: Bundle<Resources, pipe::Data<Resources>>)
                                -> VertexComponent {
        VertexComponent {
            vertices: vertices,
            translation: [0.0, 0.0],
            rotation: 0.0,
            transform: IDENTITY,
            bundle: bundle,
            update_model: true,
        }
    }

    pub fn create_draw_object(&mut self, vertices: Vec<Vertex>) -> VertexComponent {
        let pso = self.factory
            .create_pipeline_simple(include_bytes!("shader/triangle_150.glslv"),
                                    include_bytes!("shader/triangle_150.glslf"),
                                    pipe::new())
            .unwrap();

        let vertex_count = vertices.len();

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


        return DrawSystem::new_vertex_component(vertices, Bundle::new(slice, pso, data));
    }

    pub fn create_draw_object_stl(&mut self, stl: &'static [u8], color: Color) -> VertexComponent {
        let pso = self.factory
            .create_pipeline_simple(include_bytes!("shader/triangle_150.glslv"),
                                    include_bytes!("shader/triangle_150.glslf"),
                                    pipe::new())
            .unwrap();
        use std::io::Cursor;

        let mut model_reader = Cursor::new(stl.iter());

        let stl_file = stl::read_stl(&mut model_reader).expect("Failed to load model");
        let vertex_count = stl_file.header.num_triangles * 3;

        let vertices =
            stl_file.triangles.iter().flat_map(|t| Point::from_stl(t, color)).collect();

        let vertex_buffer = self.factory
            .create_buffer(vertex_count as usize,
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

        return DrawSystem::new_vertex_component(vertices, Bundle::new(slice, pso, data));
    }

    pub fn create_text(&self) -> TextComponent {
        let normal_text = gfx_text::new(self.factory.clone()).with_size(60).unwrap();
        return TextComponent {
            text: "".to_owned(),
            color: [1.0, 1.0, 1.0],
            transform: IDENTITY,
            renderer: normal_text,
        };
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

    pub fn draw(&mut self, object: &mut DrawComponent) -> () {
        object.draw(self.resize, &mut self.encoder, &self. color_view)
    }
}
