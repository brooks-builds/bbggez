extern crate bbggez;

use bbggez::ggez::{
	conf::Conf,
	event,
	event::EventHandler,
	graphics,
	nalgebra::{Point2, Vector2},
	timer, Context, ContextBuilder, GameResult,
};
use bbggez::mesh::create_circle;
use bbggez::traits::NatureOfCode;

struct Game {
	mover: Mover,
	gravity_force: Vector2<f32>,
}

impl Game {
	pub fn new(context: &mut Context) -> Game {
		let (arena_width, arena_height) = graphics::drawable_size(context);
		let mover = Mover::new(arena_width / 2.0, arena_height / 2.0, 50.0, context);
		let gravity_force = Vector2::new(0.0, 1.0);

		Game {
			mover,
			gravity_force,
		}
	}
}

impl EventHandler for Game {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
		let delta_time = timer::delta(context).as_secs_f32();
		let (_arena_width, arena_height) = graphics::drawable_size(context);

		self.mover.apply_force(self.gravity_force);

		if self.mover.location.y > arena_height {
			self.mover.location.y = arena_height;
			self.mover.reverse_vertical_velocity();
		}
		self.mover.update(delta_time);

		Ok(())
	}

	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::BLACK);

		graphics::draw(
			context,
			&self.mover.mesh,
			graphics::DrawParam::new().dest(Point2::from(self.mover.location)),
		)?;

		graphics::present(context)
	}
}

struct Mover {
	pub location: Vector2<f32>,
	pub mesh: graphics::Mesh,
	velocity: Vector2<f32>,
	acceleration: Vector2<f32>,
}

impl NatureOfCode for Mover {
	fn new(x: f32, y: f32, mass: f32, context: &mut Context) -> Mover {
		let location = Vector2::new(x, y);
		let mesh = create_circle(0.0, 0.0, mass, graphics::WHITE, context);
		let velocity = Vector2::new(0.0, 0.0);
		let acceleration = Vector2::new(0.0, 0.0);

		Mover {
			location,
			mesh,
			velocity,
			acceleration,
		}
	}

	fn update(&mut self, delta_time: f32) {
		self.velocity += self.acceleration;
		self.location += self.velocity * delta_time;
		self.acceleration *= 0.0;
	}

	fn apply_force(&mut self, force: Vector2<f32>) {
		self.acceleration += force;
	}

	fn reverse_horizontal_velocity(&mut self) {
		self.velocity.x *= -1.0;
	}

	fn reverse_vertical_velocity(&mut self) {
		self.velocity.y *= -1.0;
	}
}

fn main() {
	let mut conf = Conf::new();
	conf.window_mode = conf
		.window_mode
		.resizable(true)
		.dimensions(1860.0 / 2.0, 1015.0);
	conf.window_setup = conf.window_setup.title("");
	let (mut context, mut event_loop) = ContextBuilder::new("", "Brookzerker")
		.conf(conf)
		.build()
		.expect("Game context was not able to be created");
	let mut game = Game::new(&mut context);

	match event::run(&mut context, &mut event_loop, &mut game) {
		Ok(_) => println!("Exited cleanly"),
		Err(error) => println!("Error occured: {}", error),
	};
}
