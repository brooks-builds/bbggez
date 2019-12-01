use ggez::nalgebra::Vector2;
use ggez::Context;

pub trait NatureOfCode {
	fn new(x: f32, y: f32, mass: f32, context: &mut Context) -> Self;

	fn update(&mut self, delta_time: f32);

	fn apply_force(&mut self, force: Vector2<f32>);

	fn reverse_horizontal_velocity(&mut self);

	fn reverse_vertical_velocity(&mut self);
}
