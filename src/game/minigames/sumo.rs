use std::f32;
use std::i16;

use std::io;
use std::io::Write;

use input::InputSystem;

use game::minigame::ComponentStore;

use game::minigame::MiniGame;
use game::minigame::create_ring;

use physics::B2Point;
use physics;
use physics::PhysicsSystem;
use game::minigame::Point;
use draw;
use draw::Transform;
use draw::Color;
use draw::DrawSystem;
use draw::DrawComponent;
use input::ID;
use input::InputEvent::{InputAdded, InputRemoved};

struct Shape {
    vertices: Vec<Point>,
    color: Color,
}

struct Player {
    alive: bool,
    dead_for: u64,
    //Steps
    deaths: u64,
    name: String,
    color: Color,
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
            ring: GameObject { components: ComponentStore { draw: vec![], physics: vec![] } },
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
            shape.vertices.iter().map(|v| draw::Point::from_point_and_color(v, color)).collect();

        let draw_object = draw_system.create_draw_object(vertices);
        let physics_object = physics_system.create_body(&shape.vertices, is_dynamic);

        self.objects.push(GameObject {
            components: ComponentStore {
                draw: vec![draw_object],
                physics: vec![physics_object],
            },
        });


        &self.objects[self.objects.len() - 1]
    }

    fn new_draw_object_stl<'a>(&'a mut self,
                               draw_system: &mut DrawSystem,
                               physics_system: &mut PhysicsSystem,
                               is_dynamic: bool)
                               -> &'a GameObject {
        let physics_object = physics_system.create_body_stl(include_bytes!("../../../models/arrow_head.stl"), is_dynamic);
        let draw_body_object = draw_system.create_draw_object_stl(include_bytes!("../../../models/arrow_head.stl"), [0.1, 0.6, 0.6]);

        self.objects.push(GameObject {
            components: ComponentStore {
                draw: vec![draw_body_object],
                physics: vec![physics_object],
            },
        });


        &self.objects[self.objects.len() - 1]
    }

    fn new_player_object(&mut self,
                         draw_system: &mut DrawSystem,
                         physics_system: &mut PhysicsSystem,
                         color: Color,
                         name: String,
                         controller_id: Option<ID>
    ) {
        let physics_object = physics_system.create_body_stl(include_bytes!("../../../models/arrow_head.stl"), true);

        let text_object = draw_system.create_text();
        let draw_body_object = draw_system.create_draw_object_stl(include_bytes!("../../../models/arrow_head.stl"), color);

        let gameobject = GameObject {
            components: ComponentStore {
                draw: vec![text_object, draw_body_object],
                physics: vec![physics_object],
            },
        };


        let player = Player {
            alive: true,
            dead_for: 0,
            deaths: 0,
            color: color,
            name: name,
            object: gameobject,
            //            text: textobject,
            controller_inst_id: controller_id,
        };

        self.players.push(player);
    }

    fn revive_player_object(&mut self,
                            draw_system: &mut DrawSystem,
                            physics_system: &mut PhysicsSystem
    ) {
        for player_object in self.players.iter_mut() {
            for old_phy_obj in player_object.object.components.physics.iter_mut() {
                physics_system.destroy_body(old_phy_obj)
            }

            let physics_object = physics_system.create_body_stl(include_bytes!("../../../models/arrow_head.stl"), true);

            let text_object = draw_system.create_text();
            let draw_body_object = draw_system.create_draw_object_stl(include_bytes!("../../../models/arrow_head.stl"), player_object.color);

            let gameobject = GameObject {
                components: ComponentStore {
                    draw: vec![draw_body_object, text_object],
                    physics: vec![physics_object],
                },
            };


            player_object.object = gameobject;
            player_object.alive = true;
        }
    }

    fn new_ring(&mut self,
                draw_system: &mut DrawSystem,
                physics_system: &mut PhysicsSystem) {
        let (draw_object, physics_object) = create_ring(0.9, 0.95, [0.8, 0.02, 0.02], draw_system, physics_system);
        self.ring = GameObject { components: ComponentStore { draw: vec![draw_object], physics: vec![physics_object] } };
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

                        for p in p_obj {
                            physics_system.destroy_body(p);
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
    components: ComponentStore,
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
                                      [-0.99, -0.99, 0.0],
                                      [-0.99, 0.99, 0.0],
                                      [-1.5, -1.0, 0.0],
                                      [-1.5, 1.0, 0.0],
                                  ],
                                  color: [0.5, 0.5, 0.5],
                              },
                              false);
        state.new_draw_object(draw,
                              physics,
                              Shape {
                                  vertices: vec![
                                      [0.99, -0.99, 0.0],
                                      [0.99, 0.99, 0.0],
                                      [1.5, -1.0, 0.0],
                                      [1.5, 1.0, 0.0],
                                  ],
                                  color: [0.5, 0.5, 0.5],
                              },
                              false);
        state.new_draw_object(draw,
                              physics,
                              Shape {
                                  vertices: vec![
                                      [-0.99, 0.99, 0.0],
                                      [0.99, 0.99, 0.0],
                                      [-1.0, 1.5, 0.0],
                                      [1.0, 1.5, 0.0],
                                  ],
                                  color: [0.5, 0.5, 0.5],
                              },
                              false);
        state.new_draw_object(draw,
                              physics,
                              Shape {
                                  vertices: vec![
                                      [0.99, -0.99, 0.0],
                                      [-0.99, -0.99, 0.0],
                                      [-1.0, -1.5, 0.0],
                                      [1.0, -1.5, 0.0],
                                  ],
                                  color: [0.5, 0.5, 0.5],
                              },
                              false);

        Sumo { state: state }
    }

    fn done(&self) -> bool {
        let single_player = match self.state.players.len() {
            1 => 1,
            _ => 0,
        };

        if self.state.players.iter().filter(|i| i.alive).count() <= 1 - single_player {
            return true;
        }

        return false;
    }

    fn step(&mut self, draw: &mut DrawSystem, physics: &mut PhysicsSystem, input: &mut InputSystem) {
        // Handle input events

        if self.done() {
            for player in self.state.players.iter() {
                print!("{:?} deaths {:?} ||", player.name, player.deaths);

                io::stdout().flush().unwrap();
            }
            println!("");
            self.state.revive_player_object(draw, physics);
        }

        'events: loop {
            match input.event() {
                Some(InputAdded(id)) => {
                    let player_colors = [
                        ([1.0, 0.0, 0.0], "Red"),
                        ([0.0, 1.0, 0.0], "Green"),
                        ([0.0, 0.0, 1.0], "Blue"),
                        ([0.0, 1.0, 1.0], "Cyan"),
                        ([1.0, 0.0, 1.0], "Magenta"),
                        ([1.0, 1.0, 0.0], "Yellow"),
                    ];

                    let (color, name) = player_colors[id as usize % player_colors.len()];
                    self.state.new_player_object(draw, physics, color, name.into(), Some(id));
                    info!("New player added to game");
                }
                Some(InputRemoved(id)) => {
                    info!("Player removal event handling");

                    self.state.remove_player_object_by_controller_id(draw, physics, id)
                }
                None => { break 'events }
            }
        }

        {
            let ring = &self.state.ring;
            let players = &mut self.state.players;
            let ring_phys = ring.components.physics.first().expect("Ring must have a physics component");

            let contacts = physics.for_collisions(&ring_phys, &mut |contact| {
                //                let mut players = &self.state.players;
                let (body_handle, fixture_handle) = contact.fixture_b();

                for player in players.iter_mut() {
                    let alive = player.alive;
                    //                    let mut draw_obj = player.object.components.draw.as_mut().unwrap();
                    for draw_obj in player.object.components.draw.iter_mut() {
                        let handle = &player.object.components.physics.first().expect("Player must have a physics component").body_handle;
                        if alive && handle == &body_handle {
                            if player.alive {
                                player.deaths = player.deaths + 1;
                            }
                            player.alive = false;
                            //TODO set text of player here;
                            draw.set_color(draw_obj,[0.05, 0.05, 0.05]);
                        }
                    }
                }
            });
        }

        for player in &mut self.state.players.iter_mut() {
            if player.alive {
                if let Some(id) = player.controller_inst_id {
                    if let Some(physics_object) = player.object.components.physics.first() {
                        if let Some(ctrlr_state) = input.get_controller_state(id) {
                            let x = (ctrlr_state.axis_l_x as f32 / i16::MAX as f32); // * 55.0;
                            let y = (ctrlr_state.axis_l_y as f32 / i16::MAX as f32) * -1.0; // * -55.0;
                            physics.apply_force_to_center([x, y, 0.0], physics_object);
                        } else {
                            info!("Input system couldn't find assigned controller {:?}", id);
                        }
                    }
                }
            } else {
                player.dead_for = player.dead_for + 1;
            }
        }

        physics.step();


        // Graphics step (just set the component inputs)
        for object in self.state.objects.iter_mut() {
            for draw_object in object.components.draw.iter_mut() {
                if let Some(ref physics_object) = object.components.physics.first() {
                    match draw_object {
                        &mut DrawComponent::Vertex { mut transform, .. } => {
                            transform = Transform {
                                transform: physics.get_transformation(&physics_object),
                            }
                        }
                        _ => ()
                    }
                }
            }
        }

        for player in &mut self.state.players {
            //            let shape = &object.drawn_shape;
            // Set the draw transform matrix for each object
            if let Some(ref physics_object) = player.object.components.physics.first() {
                for draw_object in player.object.components.draw.iter_mut() {
                    match draw_object {
                        &mut DrawComponent::Vertex { ref mut transform, .. } => {
                            transform.transform = physics.get_transformation(&physics_object);
                        }
                        _ => ()
                    }
                }
            }
        }
    }

    fn render(&mut self, draw_system: &mut DrawSystem) -> () {
        for draw_object in &mut self.state.ring.components.draw {
            draw_system.draw(draw_object)
        }

        for object in &mut self.state.objects {
            for draw_object in &mut object.components.draw {
                draw_system.draw(draw_object)
            }
        }

        for player in &mut self.state.players.iter_mut() {
            for draw_object in &mut player.object.components.draw {
                draw_system.draw(draw_object)
            }
        }
    }
}
