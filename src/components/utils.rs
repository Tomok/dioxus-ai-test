/// Converts polar coordinates to Cartesian coordinates
pub fn polar_to_cartesian(
    radius: f32,
    angle_in_radians: f32,
    center_x: f32,
    center_y: f32,
) -> (f32, f32) {
    let x = center_x + radius * angle_in_radians.cos();
    let y = center_y + radius * angle_in_radians.sin();
    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_polar_to_cartesian() {
        // Test basic conversions
        let center_x = 100.0;
        let center_y = 100.0;

        // Right direction (0 degrees)
        let (x, y) = polar_to_cartesian(10.0, 0.0, center_x, center_y);
        assert_eq!(x as i32, 110);
        assert_eq!(y as i32, 100);

        // Up direction (270 degrees or -90 degrees)
        let (x, y) = polar_to_cartesian(10.0, -PI / 2.0, center_x, center_y);
        assert_eq!(x as i32, 100);
        assert_eq!(y as i32, 90);

        // Left direction (180 degrees)
        let (x, y) = polar_to_cartesian(10.0, PI, center_x, center_y);
        assert_eq!(x as i32, 90);
        assert_eq!(y as i32, 100);

        // Down direction (90 degrees)
        let (x, y) = polar_to_cartesian(10.0, PI / 2.0, center_x, center_y);
        assert_eq!(x as i32, 100);
        assert_eq!(y as i32, 110);
    }
}