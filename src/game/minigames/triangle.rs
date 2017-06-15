use cgmath::Vector2;
use cgmath::{Rotation, Rotation2, Basis2};
use cgmath::Rad;
use std::f32;

use gfx;
use gfx::Encoder;
use gfx::Factory;
use gfx::traits::FactoryExt;

use game::minigame::MiniGame;
use ColorFormat;

pub struct Triangle {
    vertices: [Vertex; 3]
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

impl MiniGame for Triangle {
    fn new() -> Triangle {
        return Triangle {
            vertices: [Vertex {
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
                                         }]
        };
    }

    fn step(&mut self) -> () {
        let rot: Basis2<f32> = Rotation2::from_angle(Rad(0.01f32 * f32::consts::PI));

        let new_verts: Vec<Vertex> = self.vertices.iter()
            .map(|x| {
                let initial: Vector2<f32> = Vector2 {
                    x: x.pos[0],
                    y: x.pos[1],
                };
                let rotated = rot.rotate_vector(initial);

                Vertex {
                    pos: [rotated.x, rotated.y],
                    color: x.color,
                }
            })
            .collect();

        for i in 0..3 {
            self.vertices[i] = new_verts[i]
        }
    }

    fn render<R, F, C>(&self,
                       encoder: &mut Encoder<R, C>,
                       factory: &mut F,
                       out: &gfx::handle::RenderTargetView<R, ColorFormat>)
                       -> ()
        where R: gfx::Resources,
              F: gfx::Factory<R>,
              C: gfx::CommandBuffer<R>
    {
        const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

        let pso = factory.create_pipeline_simple(include_bytes!("../../shader/triangle_120.glslv"),
                                    include_bytes!("../../shader/triangle_120.glslf"),
                                    pipe::new())
            .unwrap();


        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&self.vertices, ());
        // encoder.update_buffer(&vertex_buffer, &TRIANGLE, 0).expect("Failed to update vertex buffer");
        let data = pipe::Data {
            vbuf: vertex_buffer,
            out: out.clone(),
        };

        encoder.draw(&slice, &pso, &data);
    }
}