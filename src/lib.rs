use rand::prelude::*;
use ggez::graphics::Color;

#[derive(Copy, Clone)]
pub struct Utility {
	rng: ThreadRng
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
		Color::from_rgb(self.rng.gen_range(175, 255), self.rng.gen_range(175, 255), self.rng.gen_range(175, 255))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn creates_a_random_color() {
        let mut utility = Utility::new();
		let color = utility.random_bright_color();

		assert!(color.r >= 0.7 && color.r <= 1.0);
		assert!(color.g >= 0.7 && color.g <= 1.0);
		assert!(color.b >= 0.7 && color.b <= 1.0);
		assert_eq!(color.a, 1.0);
    }
}