use crate::rand::prelude::Rng;

/// Returns a random (x, y) tuple that is located within the specified width and height.
///
/// The x value will be in the range [0, `width`), i.e. inclusive of 0 and exclusive of `width`.
/// The y value will be in the range [0, `height`), i.e. inclusive of 0 and exclusive of `height`.
///
/// # Arguments
/// * `width` - The width of the area
/// * `height` - The height of the area
///
/// # Example
///
/// ```
/// use bbggez::utils::random_location;
///
/// let width = 100.0;
/// let height = 100.0;
/// let (x, y) = random_location(width, height);
/// println!("X: {}, Y: {}", x, y);
/// ```
pub fn random_location(width: f32, height: f32) -> (f32, f32) {
  let mut rng = rand::thread_rng();
  let area_x: f32 = rng.gen_range(0.0f32, width);
  let area_y: f32 = rng.gen_range(0.0f32, height);

  (area_x, area_y)
}

//
//  TEST
//

#[cfg(test)]
#[test]
fn generates_random_location() {
  let width = 100.0;
  let height = 100.0;

  let (first_x, first_y) = random_location(width, height);
  let (second_x, second_y) = random_location(width, height);

  assert_ne!(first_x, second_x);
  assert_ne!(first_y, second_y);
  assert!(first_x < width && first_x >= 0f32);
  assert!(first_y < height && first_y >= 0f32);
  assert!(second_x < width && second_x >= 0f32);
  assert!(second_y < height && second_y >= 0f32);
}
