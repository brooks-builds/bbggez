use ggez::{
	graphics::Color
};

use rand::prelude::*;


pub fn random_bright_color() -> Color {
  let mut rng = rand::thread_rng();

  Color::from_rgb(
    rng.gen_range(175, 255),
    rng.gen_range(175, 255),
    rng.gen_range(175, 255),
  )
}

pub fn random_dark_color() -> Color {
  let mut rng = rand::thread_rng();

  Color::new(
    rng.gen_range(0.0, 0.51),
    rng.gen_range(0.0, 0.51),
    rng.gen_range(0.0, 0.51),
    1.0,
  )
}