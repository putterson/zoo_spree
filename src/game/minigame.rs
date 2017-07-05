use input::InputSystem;
use draw;
use draw::DrawSystem;
use draw::DrawObject;
use physics::PhysicsSystem;
use physics::PhysicsObject;
use physics::B2Point;
use std;

pub trait MiniGame

{
    fn new(draw: &mut DrawSystem, physics: &mut PhysicsSystem, input: &InputSystem) -> Self;
    fn step(&mut self, draw: &mut DrawSystem, physics: &mut PhysicsSystem, input: &mut InputSystem) -> ();
    fn render(&mut self, draw: &mut DrawSystem) -> ();
}

//World types
pub type Point = [f32; 3];
pub type Color = [f32; 3];

pub fn create_ring(id: f32, od: f32, color: Color, draw: &mut DrawSystem, physics: &mut PhysicsSystem) -> (DrawObject, PhysicsObject) {
    let pi = std::f32::consts::PI;

    let mut vertices: Vec<draw::Point> = vec![];
    let mut bounding: Vec<Point> = vec![];

    //Number of points along the edge
    let steps: i32 = 64;
    let angle_step = (2.0 * pi) / steps as f32;
    let bounding_diameter = od;

    //Physics bounds
    for n in 0..steps {
        let angle = angle_step * n as f32;
        bounding.push([angle.sin() * bounding_diameter, angle.cos() * bounding_diameter, 0.0]);
    }

    //Graphics vertices
    for n in 0..steps {
        let angle = angle_step * n as f32;
        let angle_prime = angle_step * (n + 1) as f32;

        let z = 0f32;

        let p1 = draw::Point {
            pos: [angle.sin() * id, angle.cos() * id, z],
            color: color,

        };

        let p2 = draw::Point {
            pos: [angle.sin() * od, angle.cos() * od, z],
            color: color,
        };

        let p3 = draw::Point {
            pos: [angle_prime.sin() * id, angle_prime.cos() * id, z],
            color: color,
        };

        vertices.push(p1);
        vertices.push(p2);
        vertices.push(p3);

        let p1 = draw::Point {
            pos: [angle_prime.sin() * od, angle_prime.cos() * od, z],
            color: color,

        };

        vertices.push(p1);
        vertices.push(p2);
        vertices.push(p3);
    }

    let draw_object = draw.create_draw_object(vertices);
    let physics_object = physics.create_boundary_sensor(&bounding);

    return (draw_object, physics_object);
}