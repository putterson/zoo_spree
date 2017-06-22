extern crate wrapped2d;

use cgmath::Vector2;
use cgmath::{Rotation, Rotation2, Basis2};
use cgmath::Rad;
use std::f32;
use std::i16;

use self::wrapped2d::b2;
use self::wrapped2d::handle::TypedHandle;
use self::wrapped2d::user_data::NoUserData;

use input::InputState;

use Components;

use game::minigame::MiniGame;

use physics::B2Point;
use draw::Point;
use draw::Transform;
use draw::Color;
use draw::DrawSystem;
use draw::DrawObject;

struct Shape {
    vertices: Vec<B2Point>,
    color: Color,
}

struct GameState {
    objects: Vec<GameObject>,
    b2world: b2::World<NoUserData>,
}

impl GameState {
    fn new() -> GameState {
        let gravity = B2Point { x: 0., y: -10.0 };
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
                B2Point {x: -1.0, y: -1.0},
                B2Point {x: 1.0, y: -1.0},
                B2Point {x: 0.0, y: -2.0},
            ],
            color: [0.0, 1.0, 0.0],
        };
        self.new_object(shape,
                        // Static object
                        false);
    }
    fn new_draw_object<'a>(&'a mut self,
                              draw_system: &mut DrawSystem,
                              shape: Shape,
                              is_dynamic: bool)
                              -> &'a GameObject {
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

        let length = shape.vertices.len();
        let color = shape.color;
        let vertices = shape.vertices.iter().map(|v| Point::from_point_and_color(v, color)).collect();

        self.objects.push(GameObject {
            drawn_shape: shape,
            body: body_handle,
            components: Components {
                draw: Some(draw_system.create_draw_object(vertices, length)),
            },
        });


        &self.objects[self.objects.len() - 1]
    }

    fn new_object(&mut self, shape: Shape, is_dynamic: bool) -> &GameObject {
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
            components: Components { draw: None },
        });


        &self.objects[self.objects.len() - 1]
    }
}

struct GameObject {
    drawn_shape: Shape,
    // Box2D body, holds object state such as position, velocity, etc
    body: TypedHandle<b2::Body>,
    components: Components,
}

pub struct Sumo {
    state: GameState,
}

impl MiniGame for Sumo {
    fn new(draw: &mut DrawSystem) -> Sumo {
        let mut state = GameState::new();
        for i in 1..10 {
            state.new_draw_object(
                draw,
                Shape {
                                    vertices: vec![
                                            B2Point {x: -0.1, y: 0.1},
                                            B2Point {x: 0.11, y: 0.1},
                                            B2Point {x: 0.0, y: 0.0},
                                        ],
                                    color: [0.0, 0.0, 1.0],
                                },
                                true);
        }
        state.add_borders();

        Sumo { state: state }
    }

    fn step(&mut self, input: &InputState) {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        if input.controllers.len() > 0 {
            x = (input.controllers[0].axis_l_x as f32 / i16::MAX as f32) * 0.5;
            y = (input.controllers[0].axis_l_y as f32 / i16::MAX as f32) * -0.5;
        }

        self.state.step(b2::Vec2 { x: x, y: y });

        for object in &mut self.state.objects {
            let shape = &object.drawn_shape;
            let body = self.state.b2world.body_mut(object.body);
            let transform = body.transform();
            // let x = (transform.rot.cos * v.x - transform.rot.sin * v.y) + transform.pos.x;
            // let y = (transform.rot.sin * v.x + transform.rot.cos * v.y) + transform.pos.y;
            let mut transform_matrix = Transform {
                transform: 
                [[transform.rot.cos, transform.rot.sin, 0.0, 0.0],
                [-transform.rot.sin, transform.rot.cos, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [transform.pos.x, transform.pos.y, 0.0, 1.0]],
            };
            // object.transform = transform;
            match object.components.draw {
                Some(ref mut draw_object) => {
                    draw_object.transform = transform_matrix;
                }
                None => (),
            }
        }
    }

    fn render(&mut self, draw_system: &mut DrawSystem) -> () {
        for object in &mut self.state.objects {
            match object.components.draw {
                Some(ref mut draw_object) => {
                    // let draw_o : &mut DrawObject = 
                    draw_system.draw(draw_object);
                }
                None => (),
            }
        }
    }
}
