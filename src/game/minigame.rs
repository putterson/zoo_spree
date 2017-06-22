use input::InputState;
use draw::DrawSystem;

pub trait MiniGame
          
{
    fn new(draw: &mut DrawSystem) -> Self;
    fn step(&mut self, input: &InputState) -> ();
    fn render(&mut self, draw: &mut DrawSystem) -> ();
}
