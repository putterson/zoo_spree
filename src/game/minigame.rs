use input::InputState;
use draw::DrawSystem;
use physics::PhysicsSystem;

pub trait MiniGame
          
{
    fn new(draw: &mut DrawSystem, physics: &mut PhysicsSystem) -> Self;
    fn step(&mut self, input: &InputState, physics: &mut PhysicsSystem) -> ();
    fn render(&mut self, draw: &mut DrawSystem) -> ();
}
