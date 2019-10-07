use ggez::{
    conf::Conf,
    event,
    event::EventHandler,
    graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect},
    nalgebra::Point2,
    Context, ContextBuilder,
};
use palette::{Hsl, LinSrgb};
use rand::prelude::*;

pub extern crate ggez;
extern crate palette;
pub extern crate rand;

pub fn run<T: EventHandler>(game: &mut T, title: &str, author: &str) {
    let mut conf = Conf::new();
    conf.window_mode = conf
        .window_mode
        .dimensions(1860.0 / 2.0, 1015.0)
        .resizable(true);
    conf.window_setup = conf.window_setup.title(title);
    let (mut context, mut event_loop) = ContextBuilder::new(title, author)
        .conf(conf)
        .build()
        .expect("Game context was not able to be created");

    match event::run::<T>(&mut context, &mut event_loop, game) {
        Ok(_) => println!("Exited cleanly"),
        Err(error) => println!("Error occured: {}", error),
    };
}

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
        let color: LinSrgb = Hsl::new(self.rng.gen_range(0.0, 360.0), 1.0, 0.8).into();
        Color::from_rgb(
            (color.red * 255.0) as u8,
            (color.green * 255.0) as u8,
            (color.blue * 255.0) as u8,
        )
    }

    pub fn random_dark_color(&mut self) -> Color {
        let color: LinSrgb = Hsl::new(self.rng.gen_range(0.0, 360.0), 1.0, 0.2).into();
        Color::from_rgb(
            (color.red * 255.0) as u8,
            (color.green * 255.0) as u8,
            (color.blue * 255.0) as u8,
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
