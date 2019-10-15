extern crate bbggez;

use bbggez::ggez::{
	event::EventHandler, graphics, graphics::Color, nalgebra::Point2, Context, GameResult,
};

use bbggez::{color::random_bright_color, mesh::create_circle, run::run, timer::Timer};

struct Game {
	color: Color,
	timer: Timer,
}

impl Game {
	pub fn new() -> Game {
		Game {
			color: random_bright_color(),
			timer: Timer::new(1.0),
		}
	}
}

impl EventHandler for Game {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
		self.timer.update(context);
		if self.timer.is_time_up() {
			self.color = random_bright_color();
			self.timer.reset();
		}

		Ok(())
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::BLACK);

		let (width, height) = graphics::drawable_size(context);

		let circle = create_circle(width / 2.0, height / 2.0, 200.0, self.color, context);

		graphics::draw(context, &circle, (Point2::new(0.0, 0.0),))?;

		graphics::present(context)
	}
}

fn main() {
	let mut game = Game::new();

	run(&mut game, "Random Bright Colors", "bbggez")
}
