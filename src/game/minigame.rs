use input::InputSystem;
use draw::DrawSystem;
use physics::PhysicsSystem;

pub trait MiniGame
          
{
    fn new(draw: &mut DrawSystem, physics: &mut PhysicsSystem, input: &InputSystem) -> Self;
    fn step(&mut self, draw: &mut DrawSystem, physics: &mut PhysicsSystem, input: &mut InputSystem) -> ();
    fn render(&mut self, draw: &mut DrawSystem) -> ();
}
