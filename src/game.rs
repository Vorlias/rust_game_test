extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;
extern crate specs;
use crate::components::basic::*;
use crate::types::vec2f::*;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::*;
use specs::prelude::*;
use specs::{World, WorldExt};

struct TestSystem;
impl<'a> System<'a> for TestSystem {
	type SystemData = (WriteStorage<'a, Position>);

	fn run(&mut self, mut positions: Self::SystemData) {
		(&mut positions)
			.par_join()
			.for_each(|position| position.xy = position.xy + Vec2f::new(1.0, 1.0));
	}
}

pub struct Game<'a, 'b> {
	world: World,
	dispatcher: Dispatcher<'a, 'b>,
	gl: GlGraphics,
}

impl Game<'_, '_> {
	pub fn render(&mut self, args: &RenderArgs) {
		self.gl.draw(args.viewport(), |_c, gl| {
			graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
		});

		let entities = self.world.read_component::<Sprite>();
		let positions = self.world.read_component::<Position>();
		for (entity, position) in (&entities, &positions).join() {
			entity.draw(&mut self.gl, args, position.xy);
		}
	}

	pub fn update(&mut self, _args: &UpdateArgs) {
		self.dispatcher.dispatch(&self.world);
		self.world.maintain();
	}

	fn register_entities(&mut self) {
		self.world.register::<Position>();
		self.world.register::<Sprite>();
	}

	pub fn new(opengl: OpenGL) -> Game<'static, 'static> {
		let dispatcher = DispatcherBuilder::new()
			.with(TestSystem, "mvSystemTest", &[])
			.build();

		let mut game = Game {
			world: World::new(),
			dispatcher: dispatcher,
			gl: GlGraphics::new(opengl),
		};

		game.register_entities();

		for n in 1..=10 {
			game.world
				.create_entity()
				.with(Position::new(25.0 * (n as f64), 5.0))
				.with(Sprite {})
				.build();
		}

		game
	}
}
