extern crate bbggez;
use bbggez::ggez::{
  event::EventHandler,
  graphics,
  Context,
  GameResult,
};

struct Game {}

impl Game {
  fn new () -> Game {
    Game {}
  }
}

impl EventHandler for Game {
  fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
    println!("update");
    return Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx, graphics::WHITE);
    println!("draw");
    graphics::present(ctx)
  }
}

fn main() {
  let mut game = Game::new();
  bbggez::run(&mut game, "Runtime", "Brookzerker")
}
