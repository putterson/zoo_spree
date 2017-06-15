use cgmath::Vector2;
use cgmath::{Rotation, Rotation2, Basis2};
use cgmath::Rad;
use std::f32;

use gfx;
use gfx::Encoder;
use gfx::Factory;
use gfx::PipelineState;
use gfx::Resources;
use gfx::Slice;
use gfx::traits::FactoryExt;

use game::minigame::MiniGame;
use ColorFormat;

pub struct Triangle<R>
    where R: Resources
{
    vertices: Vec<Vertex>,
    pso: PipelineState<R, pipe::Meta>,
    slice: Slice<R>,
    data: pipe::Data<R>,
}

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

        let pso = factory.create_pipeline_simple(include_bytes!("../../shader/triangle_120.glslv"),
                                    include_bytes!("../../shader/triangle_120.glslf"),
                                    pipe::new())
            .unwrap();


        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertices, ());

        let data = pipe::Data {
            vbuf: vertex_buffer,
            out: out.clone(),
        };
        return Triangle {
            vertices: vertices,
            pso: pso,
            slice: slice,
            data: data,
        };
    }

    fn step(&mut self) -> () {
        let rot: Basis2<f32> = Rotation2::from_angle(Rad(0.01f32 * f32::consts::PI));

        for v in self.vertices.iter_mut() {
            let initial: Vector2<f32> = Vector2 {
                x: v.pos[0],
                y: v.pos[1],
            };
            let rotated = rot.rotate_vector(initial);

            v.pos = [rotated.x, rotated.y];
        }
        // let new_verts: Vec<Vertex> = self.vertices
        //     .iter()
        //     .map(|x| {
        //         let initial: Vector2<f32> = Vector2 {
        //             x: x.pos[0],
        //             y: x.pos[1],
        //         };
        //         let rotated = rot.rotate_vector(initial);

        //         Vertex {
        //             pos: [rotated.x, rotated.y],
        //             color: x.color,
        //         }
        //     })
        //     .collect();

        // for i in 0..3 {
        //     self.vertices[i] = new_verts[i]
        // }
    }

    fn render<C>(&self, encoder: &mut Encoder<R, C>) -> ()
        where C: gfx::CommandBuffer<R>
    {
        encoder.update_buffer(&self.data.vbuf, &self.vertices, 0)
            .expect("Failed to update vertex buffer");
        encoder.draw(&self.slice, &self.pso, &self.data);
    }
}