extern crate bbggez;

use bbggez::ggez::{
	event,
	event::EventHandler,
	graphics,
	graphics::Color,
	nalgebra::Point2,
	timer, Context, ContextBuilder, GameResult,
};

use bbggez::{
	mesh::create_circle,
	color::random_dark_color
};

struct Game {
	color: Color,
}

impl Game {
	pub fn new(_context: &mut Context) -> Game {
		Game {
			color: random_dark_color(),
		}
	}
}

impl EventHandler for Game {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
		if timer::ticks(context) % 1000 == 0 {
			self.color = random_dark_color();
		}

		Ok(())
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::WHITE);

		let (width, height) = graphics::drawable_size(context);

		let circle = create_circle(width / 2.0, height / 2.0, 200.0, self.color, context);

		graphics::draw(context, &circle, (Point2::new(0.0, 0.0),))?;

		graphics::present(context)
	}
}

fn main() {
	let (mut context, mut event_loop) = ContextBuilder::new("Random color example", "Brookzerker")
		.build()
		.expect("Game context was not able to be created");
	let mut game = Game::new(&mut context);

	match event::run(&mut context, &mut event_loop, &mut game) {
		Ok(_) => println!("Exited cleanly"),
		Err(error) => println!("Error occured: {}", error),
	};
}
