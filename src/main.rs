mod game;
use crate::components::basic::*;
use game::Game;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use specs::prelude::*;
use specs::{World, WorldExt};

pub mod components;

fn main() {
	let opengl = OpenGL::V3_2;

	// Set up the window
	let mut window: GlutinWindow = WindowSettings::new("Game", [800, 600])
		.graphics_api(opengl)
		.exit_on_esc(true)
		.build()
		.unwrap();

	let mut game = Game::new(opengl);

	// Set up the events
	let mut events = Events::new(EventSettings::new());

	// event loop
	while let Some(e) = events.next(&mut window) {
		// Updating
		if let Some(u) = e.update_args() {
			game.update(&u)
		}

		// Rendering
		if let Some(r) = e.render_args() {
			game.render(&r);
		}
	}
}
