extern crate specs;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use specs::{Component, HashMapStorage, VecStorage};
use crate::types::vec2f::Vec2f;

#[derive(Debug)]
pub struct Position {
	pub xy: Vec2f,
}

impl Position {
	pub fn zero() -> Position {
		Position { xy: Vec2f::new(0.0, 0.0) }
	}

	pub fn new(x: f64, y: f64) -> Position {
		Position { xy: Vec2f::new(x, y)}
	}
}

impl Component for Position {
	type Storage = VecStorage<Self>;
}

pub trait Drawable {
	fn draw(&self, gl: &mut GlGraphics, args: &RenderArgs, xy: Vec2f);
}

pub struct Sprite {}

impl Component for Sprite {
	type Storage = HashMapStorage<Self>;
}

impl Drawable for Sprite {
	fn draw(&self, gl: &mut GlGraphics, args: &RenderArgs, xy: Vec2f) {
		gl.draw(args.viewport(), |ctx, gl| {
			let transform = ctx.transform;
			let sq = graphics::rectangle::square(xy.x, xy.y, 20_f64);
			graphics::rectangle([1.0, 0.0, 0.0, 1.0], sq, transform, gl)
		});
	}
}
