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

use draw::Point;
use draw::Color;
use draw::DrawSystem;

struct Shape {
    vertices: Vec<Point>,
    color: Color,
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
                              draw_system: &DrawSystem,
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

        self.objects.push(GameObject {
            drawn_shape: shape,
            body: body_handle,
            components: Components {
                draw: Some(draw_system.create_draw_object(shape.vertices.len())),
            },
        });


        &self.objects[self.objects.len() - 1]
    }

    fn new_object<'a>(&'a mut self, shape: Shape, is_dynamic: bool) -> &'a GameObject {
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
    fn new(draw: &DrawSystem) -> Sumo {
        let mut state = GameState::new();
        state.new_draw_object(Shape {
                                  vertices: vec![
                                        Point {x: -0.5, y: 0.5},
                                        Point {x: 0.6, y: 0.5},
                                        Point {x: 0.0, y: 0.0},
                                    ],
                                  color: [0.0, 0.0, 1.0],
                              },
                              true);
        state.add_borders();
        Sumo { state: state }
    }

    fn step(&mut self, input: &InputState) {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        if input.controllers.len() > 0 {
            x = (input.controllers[0].axis_l_x as f32 / i16::MAX as f32) * 11.0;
            y = (input.controllers[0].axis_l_y as f32 / i16::MAX as f32) * -11.0;
        }

        self.state.step(b2::Vec2 { x: x, y: y });

        for object in &self.state.objects {
            let shape = &object.drawn_shape;
            let body = self.state.b2world.body_mut(object.body);
            let transform = body.transform();
            object.transform = transform;
        }
    }

    fn render<C>(&self, draw_system: &DrawSystem) -> () {
        for object in &self.state.objects {
            match object.components.draw {
                Some(ref draw_object) => {
                    draw_system.draw(draw_object);
                }
                None => (),
            }
        }
    }
}
