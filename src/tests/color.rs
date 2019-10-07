use crate::color::{random_bright_color, random_dark_color};

#[test]
fn creates_a_random_bright_color() {
	let color = random_bright_color();
	let another_color = random_bright_color();

	assert!(color.r + color.g + color.b >= 1.5);
	assert_eq!(color.a, 1.0);
	assert!(color.r + color.g + color.b != another_color.r + another_color.g + another_color.b);
}

#[test]
fn creates_a_random_dark_color() {
	let color = random_dark_color();
	let another_color = random_dark_color();

	assert!(color.r + color.g + color.b <= 1.5);
	assert_eq!(color.a, 1.0);
	assert!(color.r + color.g + color.b != another_color.r + another_color.g + another_color.b);
}
