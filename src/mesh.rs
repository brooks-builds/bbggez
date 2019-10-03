use ggez::{
	graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect},
	nalgebra::Point2,
	Context,
};

pub fn create_circle(
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