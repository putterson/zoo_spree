use std::f32;
use std::i16;

use std::io;
use std::io::Write;

use input::InputSystem;

use game::minigame::MiniGame;
use game::minigame::create_ring;

use physics::PhysicsSystem;
use physics::PhysicsObject;
use game::minigame::Point;
use draw;
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
    draw_component: DrawComponent,
    death_count_text: DrawComponent,
    physics_component: PhysicsObject,
    //    object: GameObject,
}

struct GameState {
    walls: Vec<Wall>,
    ring: Ring,
    players: Vec<Player>,
}

struct Ring {
    draw_component: DrawComponent,
    physics_component: PhysicsObject,
}

struct Wall {
    draw_component: DrawComponent,
    physics_component: PhysicsObject,
}

impl GameState {
    fn step(&mut self) {}

    fn new_wall(draw_system: &mut DrawSystem,
                physics_system: &mut PhysicsSystem,
                shape: Shape,
                is_dynamic: bool)
                -> Wall {
        let color = shape.color;
        let vertices =
            shape.vertices.iter().map(|v| draw::Point::from_point_and_color(v, color)).collect();

        let draw_object = draw_system.create_draw_object(vertices);
        let physics_object = physics_system.create_body(&shape.vertices, is_dynamic);

        Wall {
            draw_component: draw_object,
            physics_component: physics_object,
        }
    }

    fn new_player_object(draw_system: &mut DrawSystem,
                         physics_system: &mut PhysicsSystem,
                         color: Color,
                         name: String,
                         controller_id: Option<ID>
    ) -> Player {
        let physics_object = physics_system.create_body_stl(include_bytes!("../../../models/arrow_head.stl"), true);

        let text_object = draw_system.create_text();
        let draw_body_object = draw_system.create_draw_object_stl(include_bytes!("../../../models/arrow_head.stl"), color);

        Player {
            alive: true,
            dead_for: 0,
            deaths: 0,
            color: color,
            name: name,
            draw_component: draw_body_object,
            death_count_text: text_object,
            physics_component: physics_object,
            controller_inst_id: controller_id,
        }
    }

    fn revive_player_object(&mut self,
                            draw_system: &mut DrawSystem,
                            physics_system: &mut PhysicsSystem
    ) {
        for player_object in self.players.iter_mut() {
            physics_system.destroy_body(&player_object.physics_component);

            let physics_object = physics_system.create_body_stl(include_bytes!("../../../models/arrow_head.stl"), true);

            let text_object = draw_system.create_text();
            let draw_body_object = draw_system.create_draw_object_stl(include_bytes!("../../../models/arrow_head.stl"), player_object.color);

            player_object.draw_component = draw_body_object;
            player_object.death_count_text = text_object;
            player_object.physics_component = physics_object;

            player_object.alive = true;
        }
    }

    fn new_ring(draw_system: &mut DrawSystem,
                physics_system: &mut PhysicsSystem) -> Ring {
        let (draw_object, physics_object) = create_ring(0.9, 0.95, [0.8, 0.02, 0.02], draw_system, physics_system);
        Ring {
            draw_component: draw_object,
            physics_component: physics_object,
        }
    }

    fn remove_player_object_by_controller_id(&mut self,
                                             draw_system: &mut DrawSystem,
                                             physics_system: &mut PhysicsSystem,
                                             id: ID) {
        self.players.retain(|ref p| {
            match p.controller_inst_id {
                Some(player_controller_id) => {
                    if id == player_controller_id {
                        let p_obj = &p.physics_component;
                        physics_system.destroy_body(p_obj);

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

pub struct Sumo {
    state: GameState,
}

impl MiniGame for Sumo {
    fn new(draw: &mut DrawSystem, physics: &mut PhysicsSystem, input: &InputSystem) -> Sumo {
        let ring = GameState::new_ring(draw, physics);

        let mut walls: Vec<Wall> = vec![];

        walls.push(GameState::new_wall(draw,
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
                                       false));
        walls.push(GameState::new_wall(draw,
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
                                       false));
        walls.push(GameState::new_wall(draw,
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
                                       false));
        walls.push(GameState::new_wall(draw,
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
                                       false));

        Sumo {
            state: GameState {
                walls: walls,
                ring: ring,
                players: vec![],
            }
        }
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
                    self.state.players.push(GameState::new_player_object(draw, physics, color, name.into(), Some(id)));
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
            let ring_phys = &ring.physics_component;

            let contacts = physics.for_collisions(ring_phys, &mut |contact| {
                //                let mut players = &self.state.players;
                let (body_handle, fixture_handle) = contact.fixture_b();

                for player in players.iter_mut() {
                    let alive = player.alive;
                    let handle = &player.physics_component.body_handle;
                    if alive && handle == &body_handle {
                        if player.alive {
                            player.deaths = player.deaths + 1;
                        }
                        player.alive = false;
                        //TODO set text of player here;
                        let draw_obj = &mut player.draw_component;
                        draw.set_color(draw_obj, [0.05, 0.05, 0.05]);
                    }
                }
            });
        }

        for player in &mut self.state.players.iter_mut() {
            if player.alive {
                if let Some(id) = player.controller_inst_id {
                    if let Some(ctrlr_state) = input.get_controller_state(id) {
                        let x = (ctrlr_state.axis_l_x as f32 / i16::MAX as f32); // * 55.0;
                        let y = (ctrlr_state.axis_l_y as f32 / i16::MAX as f32) * -1.0; // * -55.0;
                        physics.apply_force_to_center([x, y, 0.0], &player.physics_component);
                    } else {
                        info!("Input system couldn't find assigned controller {:?}", id);
                    }
                }
            } else {
                player.dead_for = player.dead_for + 1;
            }
        }


        physics.step();


        // Graphics step (just set the component inputs)
        for player in self.state.players.iter_mut() {
            //            let shape = &object.drawn_shape;
            // Set the draw transform matrix for each object
            match &mut player.draw_component {
                &mut DrawComponent::Vertex { ref mut transform, .. } => {
                    transform.transform = physics.get_transformation(&player.physics_component);
                }
                _ => ()
            }
        }
    }


    fn render(&mut self, draw_system: &mut DrawSystem) -> () {
        draw_system.draw(&mut self.state.ring.draw_component);

        for wall in self.state.walls.iter_mut() {
            draw_system.draw(&mut wall.draw_component);
        }

        for player in &mut self.state.players.iter_mut() {
            draw_system.draw(&mut player.draw_component)
        }
    }
}
