#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

pub fn convex_hull(mut points: Vec<Point>) -> Vec<Point> {
    points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap().then_with(|| a.y.partial_cmp(&b.y).unwrap()));

    let mut lower = Vec::new();
    for p in &points {
        while lower.len() >= 2 && cross(&lower[lower.len() - 2], &lower[lower.len() - 1], p) <= 0.0 {
            lower.pop();
        }
        lower.push(*p);
    }

    let mut upper = Vec::new();
    for p in points.iter().rev() {
        while upper.len() >= 2 && cross(&upper[upper.len() - 2], &upper[upper.len() - 1], p) <= 0.0 {
            upper.pop();
        }
        upper.push(*p);
    }

    lower.pop();
    upper.pop();
    lower.append(&mut upper);
    lower
}

fn cross(o: &Point, a: &Point, b: &Point) -> f64 {
    (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
}

pub fn min_area_rectangle(points: Vec<Point>) -> f64 {
    let hull = convex_hull(points);
    let n = hull.len();
    if n < 3 {
        return 0.0;
    }

    let mut min_area = f64::MAX;
    let mut k = 1;

    for i in 0..n {
        while area(&hull[i], &hull[(i + 1) % n], &hull[k % n], &hull[(k + 1) % n]) < area(&hull[i], &hull[(i + 1) % n], &hull[(k + 1) % n], &hull[(k + 2) % n]) {
            k += 1;
        }
        min_area = min_area.min(area(&hull[i], &hull[(i + 1) % n], &hull[k % n], &hull[(k + 1) % n]));
    }

    min_area
}

fn area(a: &Point, b: &Point, c: &Point, d: &Point) -> f64 {
    let ab = distance(a, b);
    let bc = distance(b, c);
    let cd = distance(c, d);
    let da = distance(d, a);
    let ac = distance(a, c);
    let bd = distance(b, d);

    ((ab * bc).min(cd * da)).min(ac * bd)
}

fn distance(a: &Point, b: &Point) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}
