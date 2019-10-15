extern crate bbggez;
use bbggez::ggez::{event::EventHandler, graphics, Context, GameResult};
use bbggez::run::run_dim;

struct Game {}

impl Game {
    fn new() -> Game {
        Game {}
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        return Ok(());
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        graphics::present(ctx)
    }
}

fn main() {
    let mut game = Game::new();
    run_dim(&mut game, 400.0, 800.0, "Runtime", "bbggez")
}
