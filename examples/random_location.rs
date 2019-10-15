extern crate bbggez;

use bbggez::Utility;
use ggez::{
    event::EventHandler,
    graphics,
    graphics::Color,
    nalgebra::Point2,
    timer::{delta, duration_to_f64},
    Context, ContextBuilder, GameResult,
};

struct Game {
    utility: Utility,
    color: Color,
    timer: f64,
    point: Point2<f32>,
}

impl Game {
    pub fn new(_context: &mut Context) -> Game {
        let mut utility = Utility::new();

        Game {
            utility,
            color: utility.random_dark_color(),
            timer: 1.0,
            point: Point2::new(0.0, 0.0),
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        let (width, height) = graphics::drawable_size(context);
        self.timer = self.timer - duration_to_f64(delta(context));
        if self.timer <= 0.0 {
            let location = self.utility.random_location(width / 2.0, height / 2.0);

            self.point = Point2::new(location.x, location.y);
            self.timer = 1.0;
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);

        let circle =
            self.utility
                .create_circle(self.point.x, self.point.y, 200.0, self.color, context);

        graphics::draw(context, &circle, (self.point,))?;

        graphics::present(context)
    }
}

fn main() {
    let (mut context, mut _event_loop) =
        ContextBuilder::new("Random location example", "Brookzerker")
            .build()
            .expect("Game context was not able to be created");
    let mut game = Game::new(&mut context);
    bbggez::run(&mut game, "Random location example", "Brookzerker");
}
