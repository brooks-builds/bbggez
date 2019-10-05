// #[cfg(test)]
// mod tests {
// 	use crate::*;
// 	use crate::tests;

// 	#[test]
// 	fn creates_a_random_color() {
// 		let color = tests::Color::random_bright_color();

// 		assert!(color.r >= 0.7 && color.r <= 1.0);
// 		assert!(color.g >= 0.7 && color.g <= 1.0);
// 		assert!(color.b >= 0.7 && color.b <= 1.0);
// 		assert_eq!(color.a, 1.0);
// 	}

// 	#[test]
// 	fn creates_a_random_dark_color() {
// 		let mut utility = Utility::new();
// 		let color = utility.random_dark_color();
// 		let another_color = utility.random_dark_color();

// 		assert!(color.r < 0.5 && color.r >= 0.0);
// 		assert!(color.g < 0.5 && color.g >= 0.0);
// 		assert!(color.b < 0.5 && color.b >= 0.0);
// 		assert_eq!(color.a, 1.0);

// 		assert!(color.r != another_color.r && color.g != another_color.g && color.b != another_color.b);
// 	}
// }

use crate::color::{random_bright_color, random_dark_color};

#[test]
fn creates_a_random_bright_color() {
	let color = random_bright_color();
	let another_color = random_bright_color();

	assert!(color.r >= 0.7 && color.r <= 1.0);
	assert!(color.g >= 0.7 && color.g <= 1.0);
	assert!(color.b >= 0.7 && color.b <= 1.0);
	assert_eq!(color.a, 1.0);

	assert!(color.r != another_color.r && color.g != another_color.g && color.b != another_color.b);
}

#[test]
fn creates_a_random_dark_color() {
	let color = random_dark_color();
	let another_color = random_dark_color();

	assert!(color.r < 0.5 && color.r >= 0.0);
	assert!(color.g < 0.5 && color.g >= 0.0);
	assert!(color.b < 0.5 && color.b >= 0.0);
	assert_eq!(color.a, 1.0);

	assert!(color.r != another_color.r && color.g != another_color.g && color.b != another_color.b);
}
