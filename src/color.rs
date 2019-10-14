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


//
//  TESTS
//

#[cfg(test)]
#[test]
fn creates_a_random_bright_color() {
	let color = random_bright_color();
	let another_color = random_bright_color();

	assert!(color.r + color.g + color.b >= 1.5);
	assert_eq!(color.a, 1.0);
	assert!(color.r + color.g + color.b != another_color.r + another_color.g + another_color.b);
}

#[cfg(test)]
#[test]
fn creates_a_random_dark_color() {
	let color = random_dark_color();
	let another_color = random_dark_color();

	assert!(color.r + color.g + color.b <= 1.5);
	assert_eq!(color.a, 1.0);
	assert!(color.r + color.g + color.b != another_color.r + another_color.g + another_color.b);
}
