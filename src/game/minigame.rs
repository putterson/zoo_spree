use gfx::Factory;
use gfx::Encoder;
use gfx;

use input::InputState;

use ColorFormat;

pub trait MiniGame<R>
    where R: gfx::Resources
          
{
    fn new<F>(factory: &mut F, out: &gfx::handle::RenderTargetView<R, ColorFormat>) -> Self where F: gfx::Factory<R>;

    fn step(&mut self, input: &InputState) -> ();
    fn render<C>(&self, encoder: &mut Encoder<R, C>) -> () where C: gfx::CommandBuffer<R>;
}
