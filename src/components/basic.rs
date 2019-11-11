extern crate specs;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use specs::{Component, HashMapStorage, VecStorage};

#[derive(Debug)]
pub struct Position {
	x: f64,
	y: f64,
}

impl Position {
	pub fn zero() -> Position {
		Position { x: 0.0, y: 0.0 }
	}
	pub fn new(x: f64, y: f64) -> Position {
		Position { x, y }
	}
}

impl Component for Position {
	type Storage = VecStorage<Self>;
}

pub trait Drawable {
	fn draw(&self, gl: &mut GlGraphics, args: &RenderArgs, position: &Position);
}

pub struct Sprite {}

impl Component for Sprite {
	type Storage = HashMapStorage<Self>;
}

impl Drawable for Sprite {
	fn draw(&self, gl: &mut GlGraphics, args: &RenderArgs, position: &Position) {
		gl.draw(args.viewport(), |ctx, gl| {
			let transform = ctx.transform;
			let sq = graphics::rectangle::square(position.x, position.y, 20_f64);
			graphics::rectangle([1.0, 0.0, 0.0, 1.0], sq, transform, gl)
		});
	}
}
