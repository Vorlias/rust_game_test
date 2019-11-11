extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
extern crate specs;
use crate::components::basic::*;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;
use specs::prelude::*;
use specs::{World, WorldExt};

pub struct Game {
	world: World,
	gl: GlGraphics,
}

impl Game {
	pub fn render(&mut self, args: &RenderArgs) {
		let entities = self.world.read_component::<Sprite>();
		let positions = self.world.read_component::<Position>();
		for (entity, position) in (&entities, &positions).join() {
			entity.draw(&mut self.gl, args, position);
		}
	}

	pub fn update(&mut self, _args: &UpdateArgs) {
		self.world.maintain();
	}

	fn register_entities(&mut self) {
		self.world.register::<Position>();
		self.world.register::<Sprite>();
	}

	pub fn new(opengl: OpenGL) -> Game {
		let mut game = Game {
			world: World::new(),
			gl: GlGraphics::new(opengl),
		};

		game.register_entities();
		game.world
			.create_entity()
			.with(Position::zero())
			.with(Sprite {})
			.build();

		game
	}
}
