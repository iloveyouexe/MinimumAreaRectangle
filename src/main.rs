mod geometry;
mod utils;

use crate::geometry::{Point, min_area_rectangle};

fn main() {
    let points = vec![
        Point::new(0.0, 0.0),
        Point::new(1.0, 1.0),
        Point::new(2.0, 2.0),
        Point::new(3.0, 3.0),
        Point::new(0.0, 3.0),
        Point::new(3.0, 0.0),
        Point::new(2.0, 1.0),
    ];

    let area = min_area_rectangle(points);
    println!("Minimum Area Rectangle: {}", area);
}
