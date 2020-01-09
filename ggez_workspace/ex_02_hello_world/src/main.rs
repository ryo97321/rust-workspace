use cgmath;
use ggez;

use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::env;
use std::path;

struct MainState {
		frames: usize,
		text: graphics::Text,
}

impl MainState {
		fn new(ctx: &mut Context) -> GameResult<MainState> {
			let font = graphics::Font::new(ctx, "/BebasNeue-Regular.ttf")?;
			let text = graphics::Text::new(("Hello world!", font, 48.0));

			let s = MainState { frames: 0, text };
			Ok(s)
		}
}

impl event::EventHandler for MainState {
		fn update(&mut self, _ctx: &mut Context) -> GameResult {
				Ok(())
		}

		fn draw(&mut self, ctx: &mut Context) -> GameResult {
				graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

				let offset = self.frames as f32 / 10.0;
				let dest_point = cgmath::Point2::new(offset, offset);
				graphics::draw(ctx, &self.text, (dest_point,))?;
				graphics::present(ctx)?;

				self.frames += 1;
				if (self.frames % 100) == 0 {
						println!("FPS: {}", ggez::timer::fps(ctx));
				}

				Ok(())
		}
}

pub fn main() -> GameResult {
		let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
				let mut path = path::PathBuf::from(manifest_dir);
				path.push("resources");
				path
		} else {
				path::PathBuf::from("./resources")
		};

		let cb = ggez::ContextBuilder::new("helloworld", "ggez").add_resource_path(resource_dir);
		let (ctx, event_loop) = &mut cb.build()?;

		let state = &mut MainState::new(ctx)?;
		event::run(ctx, event_loop, state)
}
