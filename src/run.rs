use crate::ggez::{conf::Conf, event, event::EventHandler, ContextBuilder};

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

pub fn run_dim<T: EventHandler>(game: &mut T, width: f32, height: f32, title: &str, author: &str) {
    let mut conf = Conf::new();
  conf.window_mode = conf
    .window_mode
    .dimensions(width, height)
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