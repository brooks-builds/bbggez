extern crate bbggez;

use bbggez::{
  ggez::{
    graphics,
    graphics::Color,
    nalgebra::Point2,
    timer,
    GameResult,
    event::EventHandler,
    Context
  },
  run::run,
  color::random_bright_color,
  mesh::create_equilateral_triangle
};

struct Game {
    color: Color,
}

impl Game {
    pub fn new() -> Game {
        Game {
            color: random_bright_color(),
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        if timer::ticks(context) % 1000 == 0 {
            self.color = random_bright_color();
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::WHITE);

        let (width, height) = graphics::drawable_size(context);

        let triangle = create_equilateral_triangle(
            width/2.0, height/2.0, 300.0, self.color, context,
        );

        graphics::draw(context, &triangle, (Point2::new(0.0, 0.0),))?;

        graphics::present(context)
    }
}

fn main() {
    let mut game = Game::new();

    run(&mut game, "Create Triangle", "bbggez")
}