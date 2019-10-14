extern crate bbggez;

use bbggez::{
  rand,
  ggez::{
    event::EventHandler,
    graphics,
    graphics::Color,
    nalgebra::Point2,
    timer::{delta, duration_to_f64},
    Context,
    GameResult
  },
  mesh::create_circle,
  color::random_dark_color,
  run::run,
  utils::random_location,
};

struct Game {
    color: Color,
    timer: f64,
    point: Point2<f32>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            color: random_dark_color(),
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
            let mut location = random_location(width / 2.0, height / 2.0);

            if rand::random() {
                location.0 = -location.0;
            }

            if rand::random() {
                location.0 = -location.0;
            }
            self.point = Point2::new(location.0, location.0);
            self.timer = 1.0;
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