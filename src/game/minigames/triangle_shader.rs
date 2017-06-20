use cgmath::Vector2;
use cgmath::{Rotation, Rotation2, Basis2, Basis3, Rotation3, Matrix3};
use cgmath::Rad;
use std::f32;

use gfx;
use gfx::Encoder;
use gfx::Factory;
use gfx::PipelineState;
use gfx::Resources;
use gfx::Slice;
use gfx::traits::FactoryExt;

use input::InputState;
use game::minigame::MiniGame;
use ColorFormat;

pub struct Triangle<R>
    where R: Resources
{
    vertices: Vec<Vertex>,
    pso: PipelineState<R, pipe::Meta>,
    slice: Slice<R>,
    data: pipe::Data<R>,
    rotation: f32,
    transform: Transform,
}

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

impl<R> MiniGame<R> for Triangle<R>
    where R: gfx::Resources
{
    fn new<F>(factory: &mut F, out: &gfx::handle::RenderTargetView<R, ColorFormat>) -> Triangle<R>
        where F: gfx::Factory<R>
    {
        let vertices = vec![Vertex {
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

        let pso = factory.create_pipeline_simple(include_bytes!("../../shader/triangle_150.glslv"),
                                    include_bytes!("../../shader/triangle_150.glslf"),
                                    pipe::new())
            .unwrap();

        let (vertex_buffer, slice) =
            factory.create_vertex_buffer_with_slice(&vertices, ());
        let transform_buffer = factory.create_constant_buffer(1);

        let data = pipe::Data {
            vbuf: vertex_buffer,
            transform: transform_buffer,
            out: out.clone(),
        };

        return Triangle {
            vertices: vertices,
            pso: pso,
            slice: slice,
            data: data,
            rotation: 0.0,
            transform: Transform {
                transform: [[1.0, 0.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0, 0.0],
                            [0.0, 0.0, 1.0, 0.0],
                            [0.0, 0.0, 0.0, 1.0]],
            },
        };
    }

    fn step(&mut self, input: &InputState) -> () {
        self.rotation += 0.01;

        let rotbasis: Basis3<f32> = Rotation3::from_angle_z(Rad(self.rotation * f32::consts::PI));
        let rot: Matrix3<f32> = rotbasis.into();

        self.transform = Transform {
            transform: [[rot.x.x, rot.x.y, rot.x.z, 0.0],
                        [rot.y.x, rot.y.y, rot.y.z, 0.0],
                        [rot.z.x, rot.z.y, rot.z.z, 0.0],
                        [0.0, 0.0, 0.0, 1.0]],
        };
    }

    fn render<C>(&self, encoder: &mut Encoder<R, C>) -> ()
        where C: gfx::CommandBuffer<R>
    {
        encoder.update_buffer(&self.data.transform, &[self.transform], 0);
        encoder.draw(&self.slice, &self.pso, &self.data);
    }
}
