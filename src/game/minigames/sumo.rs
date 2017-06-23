use cgmath::Vector2;
use cgmath::{Rotation, Rotation2, Basis2};
use cgmath::Rad;
use std::f32;
use std::i16;


use input::InputState;

use Components;

use game::minigame::MiniGame;

use physics::B2Point;
use physics;
use physics::PhysicsSystem;
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
}

impl GameState {
    fn new() -> GameState {

        GameState { objects: vec![] }
    }

    fn step(&mut self) {}

    fn new_draw_object<'a>(&'a mut self,
                           draw_system: &mut DrawSystem,
                           physics_system: &mut PhysicsSystem,
                           shape: Shape,
                           is_dynamic: bool)
                           -> &'a GameObject {
        let length = shape.vertices.len();
        let color = shape.color;
        let vertices =
            shape.vertices.iter().map(|v| Point::from_point_and_color(v, color)).collect();

        let physics_object = Some(physics_system.create_physics_object(&shape.vertices, is_dynamic));

        self.objects.push(GameObject {
            drawn_shape: shape,
            components: Components {
                draw: Some(draw_system.create_draw_object(vertices, length)),
                physics: physics_object,
            },
        });


        &self.objects[self.objects.len() - 1]
    }
}

struct GameObject {
    drawn_shape: Shape,
    components: Components,
}

pub struct Sumo {
    state: GameState,
}

impl MiniGame for Sumo {
    fn new(draw: &mut DrawSystem, physics: &mut PhysicsSystem) -> Sumo {
        let mut state = GameState::new();
        for i in 1..10 {
            state.new_draw_object(draw,
                                  physics,
                                  Shape {
                                      vertices: vec![
                                            B2Point {x: 0.0, y: 0.0},
                                            B2Point {x: 0.0, y: 2.0},
                                            B2Point {x: 2.0, y: 0.0},
                                            B2Point {x: 3.0, y: 3.0},
                                            B2Point {x: 0.0, y: 2.0},
                                            B2Point {x: 2.0, y: 0.0},
                                        ],
                                      color: [0.0, 0.0, 1.0],
                                  },
                                  true);
        }
        state.new_draw_object(draw,
                              physics,
                              Shape {
                                  vertices: vec![
                                            B2Point {x: -9.0, y: -9.0},
                                            B2Point {x: -9.0, y: 9.0},
                                            B2Point {x: -15.0, y: 0.0},
                                        ],
                                  color: [1.0, 0.0, 0.0],
                              },
                              false);
        state.new_draw_object(draw,
                              physics,
                              Shape {
                                  vertices: vec![
                                            B2Point {x: 9.0, y: -9.0},
                                            B2Point {x: 9.0, y: 9.0},
                                            B2Point {x: 15.0, y: 0.0},
                                        ],
                                  color: [1.0, 0.0, 0.0],
                              },
                              false);
        state.new_draw_object(draw,
                              physics,
                              Shape {
                                  vertices: vec![
                                            B2Point {x: -9.0, y: 9.0},
                                            B2Point {x: 9.0, y: 9.0},
                                            B2Point {x: 0.0, y: 15.0},
                                        ],
                                  color: [1.0, 0.0, 0.0],
                              },
                              false);
        state.new_draw_object(draw,
                              physics,
                              Shape {
                                  vertices: vec![
                                            B2Point {x: 9.0, y: -9.0},
                                            B2Point {x: -9.0, y: -9.0},
                                            B2Point {x: 0.0, y: -15.0},
                                        ],
                                  color: [1.0, 0.0, 0.0],
                              },
                              false);

        Sumo { state: state }
    }

    fn step(&mut self, input: &InputState, physics_system: &mut PhysicsSystem) {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        if input.controllers.len() > 0 {
            x = (input.controllers[0].axis_l_x as f32 / i16::MAX as f32) * 55.0;
            y = (input.controllers[0].axis_l_y as f32 / i16::MAX as f32) * -55.0;
        }

        // Physics step
        for object in &mut self.state.objects {
        match object.components.physics {
            Some(ref physics_object) => {
                physics_system.apply_force_to_center(physics::Point { x: x, y: y }, physics_object);
            }
            None => (),
        }
        }

        physics_system.step();



        // Graphics step (just set the inputs)
        for object in &mut self.state.objects {
            let shape = &object.drawn_shape;
            // Set the draw transform matrix for each object
            match object.components.draw {
                Some(ref mut draw_object) => {
                    match object.components.physics {
                        Some(ref physics_object) => {
                            draw_object.transform = Transform {
                                transform: physics_system.get_transformation(&physics_object),
                            }
                        }
                        None => (),
                    }
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
