use crate::{
  ggez::graphics::Color, 
  palette::{Hsl, LinSrgb}, 
  rand::prelude::Rng,
};

pub fn random_bright_color() -> Color {
  let mut rng = rand::thread_rng();
  let color: LinSrgb = Hsl::new(rng.gen_range(0.0, 360.0), 1.0, 0.8).into();

  Color::from_rgb(
    (color.red * 255.0) as u8,
    (color.green * 255.0) as u8,
    (color.blue * 255.0) as u8,
  )
}

pub fn random_dark_color() -> Color {
  let mut rng = rand::thread_rng();
  let color: LinSrgb = Hsl::new(rng.gen_range(0.0, 360.0), 1.0, 0.2).into();

  Color::from_rgb(
    (color.red * 255.0) as u8,
    (color.green * 255.0) as u8,
    (color.blue * 255.0) as u8,
  )
}
