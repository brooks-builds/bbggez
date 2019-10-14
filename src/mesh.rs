use ggez::{
  graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect},
  nalgebra::{Point2, Translation2, Rotation2},
  Context,
};

use std::f32::consts::FRAC_PI_3;

pub fn create_circle(x: f32, y: f32, radius: f32, color: Color, ctx: &mut Context) -> Mesh {
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

pub fn create_square(x: f32, y: f32, side: f32, color: Color, ctx: &mut Context) -> Mesh {
  let rect = Rect::new(x, y, side, side);
  MeshBuilder::new()
    .rectangle(DrawMode::fill(), rect, color)
    .build(ctx)
    .unwrap()
}

pub fn create_triangle(points: &[Point2<f32>; 3], color: Color, ctx: &mut Context) -> Mesh {
  MeshBuilder::new()
    .polygon(DrawMode::fill(), points, color)
    .unwrap()
    .build(ctx)
    .unwrap()
}

fn get_equilateral_points(x: f32, y: f32, side: f32) -> [Point2<f32>; 3] {
  let radius: f32 = side / (3.0 as f32).sqrt();
  let point = Point2::new(0.0, 0.0 - radius);
  let translation = Translation2::new(x, y);
  [
    translation * point,
    translation * (Rotation2::new(2.0 * FRAC_PI_3) * point),
    translation * (Rotation2::new(4.0 * FRAC_PI_3) * point),
  ]
}

pub fn create_equilateral_triangle(x: f32, y: f32, side: f32, color: Color, ctx: &mut Context) -> Mesh {
  create_triangle(&get_equilateral_points(x, y, side), color, ctx)
}