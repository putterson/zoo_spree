extern crate wrapped2d;

use self::wrapped2d::b2;
use self::wrapped2d::handle::TypedHandle;
use self::wrapped2d::user_data::NoUserData;

use stl;

use cgmath;

pub type Point = cgmath::Point2<f32>;
pub type B2Point = b2::Vec2;

const SIZE_FACTOR : f32 = 10.0;

pub struct PhysicsSystem {
    world: b2::World<NoUserData>,
}

// Identity matrix
const IDENTITY: [[f32; 4]; 4] =
    [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]];

impl PhysicsSystem {
    pub fn new() -> PhysicsSystem {
        let gravity = B2Point { x: 0., y: -10.0 };
        let mut world = b2::World::<NoUserData>::new(&gravity);

        PhysicsSystem { world: world }
    }

    pub fn step(&mut self) -> () {
        self.world.step(1. / 60., 6, 2);
    }

    pub fn create_body(&mut self, vertices: &Vec<B2Point>, is_dynamic: bool) -> PhysicsObject {
        let mut body_def = b2::BodyDef::new();
        if is_dynamic {
            body_def.body_type = b2::BodyType::Dynamic;
        }

        let body_handle: TypedHandle<b2::Body> = self.world.create_body(&body_def);
        let body_box = b2::PolygonShape::new_with(&vertices);

        let mut fixture_def = b2::FixtureDef::new();
        fixture_def.density = 0.1;
        fixture_def.friction = 0.3;
        self.world.body_mut(body_handle).create_fixture(&body_box, &mut fixture_def);

        return PhysicsObject {
            transform: IDENTITY,
            body_handle: body_handle,
        };
    }

    pub fn create_body_stl(&mut self, is_dynamic: bool) -> PhysicsObject {
        let mut body_def = b2::BodyDef::new();
        if is_dynamic {
            body_def.body_type = b2::BodyType::Dynamic;
        }
        use std::io::Cursor;

        let mut model_reader = Cursor::new(include_bytes!("../models/arrow_head.stl").iter());

        let stl_file = stl::read_stl(&mut model_reader).expect("Failed to load model");


        let body_handle: TypedHandle<b2::Body> = self.world.create_body(&body_def);


        let mut fixture_def = b2::FixtureDef::new();
        fixture_def.density = 0.1;
        fixture_def.friction = 0.3;

        for t in stl_file.triangles.iter() {
            let points = [b2::Vec2 {
                              x: t.v1[0] * SIZE_FACTOR,
                              y: t.v1[1] * SIZE_FACTOR,
                          },
                          b2::Vec2 {
                              x: t.v2[0] * SIZE_FACTOR,
                              y: t.v2[1] * SIZE_FACTOR,
                          },
                          b2::Vec2 {
                              x: t.v3[0] * SIZE_FACTOR,
                              y: t.v3[1] * SIZE_FACTOR,
                          }];
            let body_box = b2::PolygonShape::new_with(&points);
            self.world.body_mut(body_handle).create_fixture(&body_box, &mut fixture_def);
        }

        return PhysicsObject {
            transform: IDENTITY,
            body_handle: body_handle,
        };
    }

    pub fn apply_force_to_center(&self, force: Point, physics_object: &PhysicsObject) {
        let force_vec = &b2::Vec2 {
            x: force.x,
            y: force.y,
        };
        self.world.body_mut(physics_object.body_handle).apply_force_to_center(force_vec, true);
    }

    pub fn get_transformation(&self, physics_object: &PhysicsObject) -> [[f32; 4]; 4] {
        // Update transformation matrix
        let body = self.world.body_mut(physics_object.body_handle);
        let transform = body.transform();
        let mut transform_matrix = [[transform.rot.cos, transform.rot.sin, 0.0, 0.0],
                                    [-transform.rot.sin, transform.rot.cos, 0.0, 0.0],
                                    [0.0, 0.0, 1.0, 0.0],
                                    [transform.pos.x / SIZE_FACTOR, transform.pos.y / SIZE_FACTOR, 0.0, 1.0]];

        return transform_matrix;
    }
}

pub struct PhysicsObject {
    transform: [[f32; 4]; 4],
    body_handle: TypedHandle<b2::Body>,
}