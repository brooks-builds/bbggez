pub extern crate ggez;
extern crate palette;

use rand::prelude::*;
use ggez::graphics::Color;
use palette::{Hsl, rgb::LinSrgb};

#[derive(Copy, Clone)]
pub struct Utility {
	pub rng: ThreadRng
}

/// the main utility that we are using for everything
/// store this in your game state so you can use the stored random number generator over and over again
impl Utility {
	pub fn new() -> Utility {
		Utility {
			rng: rand::thread_rng()
		}
	}

	/// Creates a random color with a minimum of 200 for red, green, and blue (on a range of 0-255) to ensure that
	/// the colors are bright enough for darker backgrounds
	pub fn random_bright_color(&mut self) -> Color {
    let color: LinSrgb = Hsl::new(self.rng.gen_range(0.0, 360.0), 1.0, 0.8).into();
		Color::from_rgb((color.red * 255.0) as u8, (color.green * 255.0) as u8, (color.blue * 255.0) as u8)
	}

	pub fn random_dark_color(&mut self) -> Color {
    let color: LinSrgb = Hsl::new(self.rng.gen_range(0.0, 360.0), 1.0, 0.2).into();
		Color::from_rgb((color.red * 255.0) as u8, (color.green * 255.0) as u8, (color.blue * 255.0) as u8)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn creates_a_random_color() {
        let mut utility = Utility::new();
		let color = utility.random_bright_color();
		let another_color = utility.random_dark_color();

		assert!(color.r + color.g + color.b >= 1.5);
		assert_eq!(color.a, 1.0);
		assert!(color.r + color.g + color.b != another_color.r + another_color.g + another_color.b);
    }

	#[test]
	fn creates_a_random_dark_color() {
		let mut utility = Utility::new();
		let color = utility.random_dark_color();
		let another_color = utility.random_dark_color();

		assert!(color.r + color.g + color.b <= 1.5);
		assert_eq!(color.a, 1.0);

		assert!(color.r + color.g + color.b != another_color.r + another_color.g + another_color.b);
	}
}
