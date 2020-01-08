struct State {}

impl ggez::event::EventHandler for State {
	fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
		Ok(())
	}
	fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
		Ok(())
	}
}

use ggez::*;

pub fn main() {
    let state = &mut State {};

	let c = conf::Conf::new();
	let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
		.conf(c)
		.build()
		.unwrap();

	event::run(ctx, event_loop, state).unwrap();
}
