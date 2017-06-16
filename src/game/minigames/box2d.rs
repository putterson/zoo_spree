extern crate wrapped2d; 

use cgmath::Vector2;
use cgmath::{Rotation, Rotation2, Basis2};
use cgmath::Rad;
use std::f32;
use std::collections::HashMap;

use gfx;
use gfx::Encoder;
use gfx::Factory;
use gfx::traits::FactoryExt;

use self::wrapped2d::b2;
use self::wrapped2d::handle::TypedHandle;
use self::wrapped2d::user_data::NoUserData;

use game::minigame::MiniGame;
use ColorFormat;

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

// TODO make these common type declarations?
type Point = b2::Vec2;
type Color = [f32; 3];

struct Shape {
    vertices: Vec<Point>,
    color: Color
}

impl Shape {
    fn gfx_vertices(&self, transform: &b2::Transform) -> Vec<Vertex> {
        self.vertices.clone().into_iter().map(|v| {
            let x = (transform.rot.cos * v.x - transform.rot.sin * v.y) + transform.pos.x;
            let y = (transform.rot.sin * v.x + transform.rot.cos * v.y) + transform.pos.y;
            Vertex{
                pos : [x, y],
                color: self.color
            }
        }).collect()
    }
}

struct GameState {
    objects: Vec<GameObject>,
    b2world: b2::World<NoUserData>,
}

impl GameState {
    fn new() -> GameState {
        let gravity = Point { x: 0., y: -10.0 };
        let mut world = b2::World::<NoUserData>::new(&gravity);
        GameState {
            objects: vec![],
            b2world: world
        }
    }
    fn step(&mut self) {
        self.b2world.step(1./60., 6, 2);
    }
    fn add_borders(&mut self) {
        let shape = Shape {
            vertices: vec![
                Point {x: -1.0, y: -1.0},
                Point {x: 1.0, y: -1.0},
                Point {x: 0.0, y: 0.5},
            ],
            color: [0.0,1.0,0.0]
        };
        self.new_object(shape, /*Static object*/ false);
    }
    fn new_object<'a>(&'a mut self, shape: Shape, is_dynamic: bool) -> &'a GameObject{
        let mut body_def = b2::BodyDef::new();
        if is_dynamic {
            body_def.body_type = b2::BodyType::Dynamic;
        }
        let body_handle: TypedHandle<b2::Body> = self.b2world.create_body(&body_def);
        let body_box = b2::PolygonShape::new_with(&shape.vertices);

        let mut fixture_def = b2::FixtureDef::new();
        fixture_def.density = 1.;
        fixture_def.friction = 0.3;
        self.b2world.body_mut(body_handle).create_fixture(&body_box, &mut fixture_def);

        self.objects.push(GameObject {
            drawn_shape: shape,
            body: body_handle
        });
        &self.objects[self.objects.len() - 1]
    }

}

struct GameObject {
    drawn_shape: Shape,
    // Box2D body, holds object state such as position, velocity, etc
    body: TypedHandle<b2::Body>
}

impl GameObject {
}

pub struct Box2DTestGame {
    state: GameState
}

impl MiniGame for Box2DTestGame {
    fn new() -> Box2DTestGame {
        let mut state: GameState = GameState::new();
        state.new_object(Shape {
            vertices: vec![
                Point {x: -0.5, y: -0.5},
                Point {x: 0.5, y: -0.5},
                Point {x: 0.0, y: 0.2},
            ], 
            color: [0.0, 0.0, 1.0]
        }, /*Dynamic object:*/ true); 
        state.add_borders();
        Box2DTestGame {
            state: state 
        }
    }

    fn step(&mut self) {
        self.state.step();
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

        for object in &self.state.objects { 
            let shape = &object.drawn_shape;
            let body = self.state.b2world.body_mut(object.body);
            let transform = body.transform();
            let new_verts = shape.gfx_vertices(transform);
            let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&new_verts[..], ());
            // encoder.update_buffer(&vertex_buffer, &TRIANGLE, 0).expect("Failed to update vertex buffer");
            let data = pipe::Data {
                vbuf: vertex_buffer,
                out: out.clone(),
            };

            encoder.draw(&slice, &pso, &data);
        }
    }
}
