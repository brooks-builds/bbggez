extern crate bbggez;

use bbggez::{
    color::random_dark_color,
    ggez::{event::EventHandler, graphics, graphics::Color, nalgebra::Point2, Context, GameResult},
    mesh::create_circle,
    run::run,
    timer::Timer,
    utils::random_location,
};

struct Game {
    color: Color,
    timer: Timer,
    point: Point2<f32>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            color: random_dark_color(),
            timer: Timer::new(1.0),
            point: Point2::new(0.0, 0.0),
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        let (width, height) = graphics::drawable_size(context);
        self.timer.update(context);
        if self.timer.is_time_up() {
            let location = random_location(width / 2.0, height / 2.0);
            self.point = Point2::new(location.x, location.y);
            self.timer.reset();
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context, graphics::BLACK);

        let circle = create_circle(self.point.x, self.point.y, 100.0, self.color, context);

        graphics::draw(context, &circle, (self.point,))?;

        graphics::present(context)
    }
}

fn main() {
    let mut game = Game::new();
    run(&mut game, "Random Location", "bbggez")
}
