use std::f32;
use std::i16;

use input::InputSystem;

use Components;

use game::minigame::MiniGame;
use game::minigame::create_ring;

use physics::B2Point;
use physics;
use physics::PhysicsSystem;
use draw::Point;
use draw::Transform;
use draw::Color;
use draw::DrawSystem;
use input::ID;
use input::InputEvent::{InputAdded, InputRemoved};

struct Shape {
    vertices: Vec<B2Point>,
    color: Color,
}

struct Player {
    controller_inst_id: Option<ID>,
    object: GameObject,
}

struct GameState {
    objects: Vec<GameObject>,
    ring: GameObject,
    players: Vec<Player>,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            objects: vec![],
            ring: GameObject { components: Components { draw: None, physics: None } },
            players: vec![],
        }
    }

    fn step(&mut self) {}

    fn new_draw_object<'a>(&'a mut self,
                           draw_system: &mut DrawSystem,
                           physics_system: &mut PhysicsSystem,
                           shape: Shape,
                           is_dynamic: bool)
                           -> &'a GameObject {
        let color = shape.color;
        let vertices =
            shape.vertices.iter().map(|v| Point::from_point_and_color(v, color)).collect();

        let physics_object = Some(physics_system.create_body(&shape.vertices, is_dynamic));

        self.objects.push(GameObject {
            components: Components {
                draw: Some(draw_system.create_draw_object(vertices)),
                physics: physics_object,
            },
        });


        &self.objects[self.objects.len() - 1]
    }

    fn new_draw_object_stl<'a>(&'a mut self,
                               draw_system: &mut DrawSystem,
                               physics_system: &mut PhysicsSystem,
                               is_dynamic: bool)
                               -> &'a GameObject {
        let physics_object = Some(physics_system.create_body_stl(include_bytes!("../../../models/arrow_head.stl"), is_dynamic));

        self.objects.push(GameObject {
            components: Components {
                draw: Some(draw_system.create_draw_object_stl(include_bytes!("../../../models/arrow_head.stl"), [0.1, 0.6, 0.6])),
                physics: physics_object,
            },
        });


        &self.objects[self.objects.len() - 1]
    }

    fn new_player_object(&mut self,
                         draw_system: &mut DrawSystem,
                         physics_system: &mut PhysicsSystem,
                         controller_id: Option<ID>
    ) {
        let physics_object = Some(physics_system.create_body_stl(include_bytes!("../../../models/arrow_head.stl"), true));

        let gameobject = GameObject {
            components: Components {
                draw: Some(draw_system.create_draw_object_stl(include_bytes!("../../../models/arrow_head.stl"), [0.3, 0.7, 0.7])),
                physics: physics_object,
            },
        };

        let player = Player {
            object: gameobject,
            controller_inst_id: controller_id,
        };

        self.players.push(player);
    }

    fn new_ring(&mut self,
                draw_system: &mut DrawSystem,
                physics_system: &mut PhysicsSystem) {
        let (draw_object, physics_object) = create_ring(0.9, 0.95, [8.0, 0.05, 0.05], draw_system, physics_system);
        self.ring = GameObject { components: Components { draw: Some(draw_object), physics: Some(physics_object) } };
    }

    fn remove_player_object_by_controller_id(&mut self,
                                             draw_system: &mut DrawSystem,
                                             physics_system: &mut PhysicsSystem,
                                             id: ID) {
        self.players.retain(|ref p| {
            match p.controller_inst_id {
                Some(player_controller_id) => {
                    if id == player_controller_id {
                        let p_obj = &p.object.components.physics;

                        if p_obj.is_some() {
                            physics_system.destroy_body(p_obj.as_ref().unwrap());
                        }

                        info!("Player removed from game");

                        return false;
                    }

                    return true;
                }
                None => true
            }
        });
    }
}

struct GameObject {
    components: Components,
}

pub struct Sumo {
    state: GameState,
}

impl MiniGame for Sumo {
    fn new(draw: &mut DrawSystem, physics: &mut PhysicsSystem, input: &InputSystem) -> Sumo {
        let mut state = GameState::new();

        state.new_ring(draw, physics);

        state.new_draw_object(draw,
                              physics,
                              Shape {
                                  vertices: vec![
                                      B2Point { x: -9.9, y: -9.9 },
                                      B2Point { x: -9.9, y: 9.9 },
                                      B2Point { x: -15.0, y: -10.0 },
                                      B2Point { x: -15.0, y: 10.0 },
                                  ],
                                  color: [0.5, 0.5, 0.5],
                              },
                              false);
        state.new_draw_object(draw,
                              physics,
                              Shape {
                                  vertices: vec![
                                      B2Point { x: 9.9, y: -9.9 },
                                      B2Point { x: 9.9, y: 9.9 },
                                      B2Point { x: 15.0, y: -10.0 },
                                      B2Point { x: 15.0, y: 10.0 },
                                  ],
                                  color: [0.5, 0.5, 0.5],
                              },
                              false);
        state.new_draw_object(draw,
                              physics,
                              Shape {
                                  vertices: vec![
                                      B2Point { x: -9.9, y: 9.9 },
                                      B2Point { x: 9.9, y: 9.9 },
                                      B2Point { x: -10.0, y: 15.0 },
                                      B2Point { x: 10.0, y: 15.0 },
                                  ],
                                  color: [0.5, 0.5, 0.5],
                              },
                              false);
        state.new_draw_object(draw,
                              physics,
                              Shape {
                                  vertices: vec![
                                      B2Point { x: 9.9, y: -9.9 },
                                      B2Point { x: -9.9, y: -9.9 },
                                      B2Point { x: -10.0, y: -15.0 },
                                      B2Point { x: 10.0, y: -15.0 },
                                  ],
                                  color: [0.5, 0.5, 0.5],
                              },
                              false);

        for controller_id in input.controller_ids() {
            state.new_player_object(draw, physics, Some(controller_id));
        }

        state.new_draw_object_stl(draw, physics, true);

        Sumo { state: state }
    }

    fn step(&mut self, draw: &mut DrawSystem, physics: &mut PhysicsSystem, input: &mut InputSystem) {
        // Handle input events

        'events: loop {
            match input.event() {
                Some(InputAdded(id)) => {
                    self.state.new_player_object(draw, physics, Some(id));
                    info!("New player added to game");
                }
                Some(InputRemoved(id)) => {
                    info!("Player removal event handling");

                    self.state.remove_player_object_by_controller_id(draw, physics, id)
                }
                None => { break 'events }
            }
        }

        for player in &mut self.state.players {
            match player.controller_inst_id {
                Some(id) => {
                    let maybe_ctrlr_state = input.get_controller_state(id);
                    match (&player.object.components.physics, maybe_ctrlr_state) {
                        (&Some(ref physics_object), Some(ctrlr_state)) => {
                            let x = (ctrlr_state.axis_l_x as f32 / i16::MAX as f32) * 55.0;
                            let y = (ctrlr_state.axis_l_y as f32 / i16::MAX as f32) * -55.0;
                            physics.apply_force_to_center(physics::Point { x: x, y: y }, physics_object);
                        }
                        _ => { info!("Player does not have physics object or input system couldn't find assigned controller") }
                    }
                }
                _ => ()
            }
        }

        physics.step();


        // Graphics step (just set the inputs)
        for object in &mut self.state.objects {
            //            let shape = &object.drawn_shape;
            // Set the draw transform matrix for each object
            match object.components.draw {
                Some(ref mut draw_object) => {
                    match object.components.physics {
                        Some(ref physics_object) => {
                            draw_object.transform = Transform {
                                transform: physics.get_transformation(&physics_object),
                            }
                        }
                        None => (),
                    }
                }
                None => (),
            }
        }

        for player in &mut self.state.players {
            //            let shape = &object.drawn_shape;
            // Set the draw transform matrix for each object
            match player.object.components.draw {
                Some(ref mut draw_object) => {
                    match player.object.components.physics {
                        Some(ref physics_object) => {
                            draw_object.transform = Transform {
                                transform: physics.get_transformation(&physics_object),
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
        match self.state.ring.components.draw {
            Some(ref mut draw_object) => {
                draw_system.draw(draw_object)
            }
            None => (),
        }

        for object in &mut self.state.objects {
            match object.components.draw {
                Some(ref mut draw_object) => {
                    // let draw_o : &mut DrawObject =
                    draw_system.draw(draw_object);
                }
                None => (),
            }
        }

        for player in &mut self.state.players {
            match player.object.components.draw {
                Some(ref mut draw_object) => {
                    // let draw_o : &mut DrawObject =
                    draw_system.draw(draw_object);
                }
                None => (),
            }
        }
    }
}
