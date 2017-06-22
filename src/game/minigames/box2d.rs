extern crate wrapped2d;

use cgmath::Vector2;
use cgmath::{Rotation, Rotation2, Basis2};
use cgmath::Rad;
use std::f32;
use std::i16;
use std::collections::HashMap;

use gfx;
use gfx::Encoder;
use gfx::Factory;
use gfx::traits::FactoryExt;
use gfx::Bundle;
use gfx::Slice;
use gfx::Resources;

use self::wrapped2d::b2;
use self::wrapped2d::handle::TypedHandle;
use self::wrapped2d::user_data::NoUserData;

use input::InputState;

use game::minigame::MiniGame;
use ColorFormat;

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


// Identity matrix
const TRANSFORM: Transform = Transform {
    transform: [[1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]],
};

// TODO make these common type declarations?
type Point = b2::Vec2;
type Color = [f32; 3];

struct Shape {
    vertices: Vec<Point>,
    color: Color,
}

impl Shape {
    fn gfx_vertices(&self, transform: &b2::Transform) -> Vec<Vertex> {
        self.vertices
            .clone()
            .into_iter()
            .map(|v| {
                let x = (transform.rot.cos * v.x - transform.rot.sin * v.y) + transform.pos.x;
                let y = (transform.rot.sin * v.x + transform.rot.cos * v.y) + transform.pos.y;
                Vertex {
                    pos: [x, y],
                    color: self.color,
                }
            })
            .collect()
    }
}

struct GameState<R>
    where R: Resources
{
    objects: Vec<GameObject<R>>,
    b2world: b2::World<NoUserData>,
}

impl<R> GameState<R>
    where R: Resources
{
    fn new() -> GameState<R> {
        let gravity = Point { x: 0., y: -10.0 };
        let mut world = b2::World::<NoUserData>::new(&gravity);
        GameState {
            objects: vec![],
            b2world: world,
        }
    }
    fn step(&mut self, force: b2::Vec2) {
        self.b2world.step(1. / 60., 6, 2);
        self.b2world.body_mut(self.objects[0].body).apply_force_to_center(&force, true);
    }
    fn add_borders(&mut self) {
        let shape = Shape {
            vertices: vec![
                Point {x: -1.0, y: -1.0},
                Point {x: 1.0, y: -1.0},
                Point {x: 0.0, y: -2.0},
            ],
            color: [0.0, 1.0, 0.0],
        };
        self.new_object(shape,
                        // Static object
                        false);
    }
    fn new_draw_object<'a, F>(&'a mut self,
                              factory: &mut F,
                              out: &gfx::handle::RenderTargetView<R, ColorFormat>,
                              shape: Shape,
                              is_dynamic: bool)
                              -> &'a GameObject<R>
        where F: gfx::Factory<R>
    {
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

        let pso = factory.create_pipeline_simple(include_bytes!("../../shader/triangle_150.glslv"),
                                    include_bytes!("../../shader/triangle_150.glslf"),
                                    pipe::new())
            .unwrap();

        let vertex_buffer = factory.create_buffer(shape.vertices.len() as usize,
                           gfx::buffer::Role::Vertex,
                           gfx::memory::Usage::Dynamic,
                           gfx::Bind::empty())
            .unwrap();


        let slice = Slice::new_match_vertex_buffer(&vertex_buffer);

        let transform_buffer = factory.create_constant_buffer(1);


        let data = pipe::Data {
            vbuf: vertex_buffer,
            transform: transform_buffer,
            out: out.clone(),
        };


        self.objects.push(GameObject {
            drawn_shape: shape,
            body: body_handle,
            draw_bundle: Some(Bundle::new(slice, pso, data)),
        });


        &self.objects[self.objects.len() - 1]
    }

    fn new_object<'a>(&'a mut self, shape: Shape, is_dynamic: bool) -> &'a GameObject<R> {
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
            body: body_handle,
            draw_bundle: None,
        });


        &self.objects[self.objects.len() - 1]
    }
}

struct GameObject<R>
    where R: gfx::Resources
{
    drawn_shape: Shape,
    // Box2D body, holds object state such as position, velocity, etc
    body: TypedHandle<b2::Body>,
    draw_bundle: Option<Bundle<R, pipe::Data<R>>>,
}

// impl<R> GameObject<R> {}

pub struct Box2DTestGame<R>
    where R: gfx::Resources
{
    state: GameState<R>,
}

impl<R> MiniGame<R> for Box2DTestGame<R>
    where R: Resources
{
    fn new<F>(factory: &mut F,
              out: &gfx::handle::RenderTargetView<R, ColorFormat>)
              -> Box2DTestGame<R>
        where F: gfx::Factory<R>
    {
        let mut state = GameState::new();
        state.new_draw_object(factory,
                              out,
                              Shape {
                                  vertices: vec![
                                        Point {x: -0.5, y: 0.5},
                                        Point {x: 0.6, y: 0.5},
                                        Point {x: 0.0, y: 0.0},
                                    ],
                                  color: [0.0, 0.0, 1.0],
                              },
                              // Dynamic object:
                              true);
        state.add_borders();
        Box2DTestGame { state: state }
    }

    fn step(&mut self, input: &InputState) {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        if input.controllers.len() > 0 {
            x = (input.controllers[0].axis_l_x as f32 / i16::MAX as f32) * 11.0;
            y = (input.controllers[0].axis_l_y as f32 / i16::MAX as f32) * 11.0;
        }

        self.state.step(b2::Vec2 { x: x, y: y });
    }

    fn resize(&mut self, new_target: &gfx::handle::RenderTargetView<R, ColorFormat> )
    {
        for mut object in &mut self.state.objects {
            match object.draw_bundle {
                Some(ref mut bundle) => {
                    bundle.data.out = new_target.clone();
                }
                None => (),
            }
        }
    }

    fn render<C>(&self, encoder: &mut Encoder<R, C>) -> ()
        where C: gfx::CommandBuffer<R>
    {
        for object in &self.state.objects {
            let shape = &object.drawn_shape;
            let body = self.state.b2world.body_mut(object.body);
            let transform = body.transform();
            let new_verts = shape.gfx_vertices(transform);

            match object.draw_bundle {
                Some(ref bundle) => {
                    encoder.update_buffer(&bundle.data.transform, &[TRANSFORM], 0);
                    encoder.update_buffer(&bundle.data.vbuf, &new_verts[..], 0)
                        .expect("Failed to update vertex buffer");
                    bundle.encode(encoder)
                }
                None => (),
            }

        }
    }
}
