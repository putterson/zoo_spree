use gfx::Factory;
use gfx::Encoder;
use gfx;

use ColorFormat;

pub trait MiniGame {
    fn new() -> Self;
    fn step(&mut self) -> ();
    fn render<R, F, C>(&self, encoder: &mut Encoder<R, C>, factory: &mut F, out: &gfx::handle::RenderTargetView<R, ColorFormat>) -> ()
        where R: gfx::Resources,
              F: gfx::Factory<R>,
              C: gfx::CommandBuffer<R>;
}