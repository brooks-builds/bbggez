use ggez::{
	graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect},
	nalgebra::Point2,
	Context,
};
use rand::prelude::*;

#[derive(Copy, Clone)]
pub struct Utility {
	rng: ThreadRng,
}

/// the main utility that we are using for everything
/// store this in your game state so you can use the stored random number generator over and over again
impl Utility {
	pub fn new() -> Utility {
		Utility {
			rng: rand::thread_rng(),
		}
	}

	/// Creates a random color with a minimum of 200 for red, green, and blue (on a range of 0-255) to ensure that
	/// the colors are bright enough for darker backgrounds
	pub fn random_bright_color(&mut self) -> Color {
		Color::from_rgb(
			self.rng.gen_range(175, 255),
			self.rng.gen_range(175, 255),
			self.rng.gen_range(175, 255),
		)
	}

	pub fn random_dark_color(&mut self) -> Color {
		Color::new(
			self.rng.gen_range(0.0, 0.51),
			self.rng.gen_range(0.0, 0.51),
			self.rng.gen_range(0.0, 0.51),
			1.0,
		)
	}

	pub fn create_circle(
		&mut self,
		x: f32,
		y: f32,
		radius: f32,
		color: Color,
		ctx: &mut Context,
	) -> Mesh {
		MeshBuilder::new()
			.circle(DrawMode::fill(), Point2::new(x, y), radius, 0.01, color)
			.build(ctx)
			.unwrap()
	}

	pub fn create_rect(
		&mut self,
		x: f32,
		y: f32,
		width: f32, 
		height: f32,
		color: Color,
		ctx: &mut Context,
	) -> Mesh {
		let rect = Rect::new(x, y, width, height);
		MeshBuilder::new()
			.rectangle(DrawMode::fill(), rect, color)
			.build(ctx)
			.unwrap()
	}

	pub fn create_square(
		&mut self,
		x: f32,
		y: f32,
		side: f32, 
		color: Color,
		ctx: &mut Context,
	) -> Mesh {
		let rect = Rect::new(x, y, side, side);
		MeshBuilder::new()
			.rectangle(DrawMode::fill(), rect, color)
			.build(ctx)
			.unwrap()
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

	#[test]
	fn creates_a_random_dark_color() {
		let mut utility = Utility::new();
		let color = utility.random_dark_color();
		let another_color = utility.random_dark_color();

		assert!(color.r < 0.5 && color.r >= 0.0);
		assert!(color.g < 0.5 && color.g >= 0.0);
		assert!(color.b < 0.5 && color.b >= 0.0);
		assert_eq!(color.a, 1.0);

		assert!(color.r != another_color.r && color.g != another_color.g && color.b != another_color.b);
	}
}
