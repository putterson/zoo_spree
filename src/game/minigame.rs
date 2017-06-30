use input::InputSystem;
use draw::DrawSystem;
use physics::PhysicsSystem;

pub trait MiniGame
          
{
    fn new(draw: &mut DrawSystem, physics: &mut PhysicsSystem, input: &InputSystem) -> Self;
    fn step(&mut self, input: &InputSystem, physics: &mut PhysicsSystem) -> ();
    fn render(&mut self, draw: &mut DrawSystem) -> ();
}
