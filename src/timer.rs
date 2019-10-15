use crate::ggez::{
  timer::{delta, duration_to_f64},
  Context,
};

pub struct Timer {
  start_time: f64,
  time_left: f64,
}

impl Timer {
  pub fn new(start_time: f64) -> Timer {
    Timer {
      start_time,
      time_left: start_time,
    }
  }

  pub fn update(&mut self, context: &Context) {
    self.time_left = self.time_left - duration_to_f64(delta(context));
  }

  pub fn is_time_up(&self) -> bool {
    self.time_left <= 0.0
  }

  pub fn reset(&mut self) {
    self.time_left = self.start_time;
  }
}
