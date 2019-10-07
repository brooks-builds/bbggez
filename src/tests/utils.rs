use crate::utils::random_location;

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