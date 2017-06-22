use input::InputState;
use draw::DrawSystem;

pub trait MiniGame
          
{
    fn new(draw: &DrawSystem) -> Self;
    fn step(&mut self, input: &InputState) -> ();
    fn render(&self, draw: &DrawSystem) -> ();
}
