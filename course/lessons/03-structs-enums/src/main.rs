//! Lesson 03: structs, enums and `match`.

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance_to(self, other: Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

/// Each variant carries its own data.
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { a: f64, b: f64, c: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        // `match` must cover every variant, or the program won't compile.
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { a, b, c } => {
                let s = (a + b + c) / 2.0; // Heron's formula
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }

    fn name(&self) -> &'static str {
        match self {
            Shape::Circle { .. } => "circle",
            Shape::Rectangle { .. } => "rectangle",
            Shape::Triangle { .. } => "triangle",
        }
    }
}

fn main() {
    let shapes = [
        Shape::Circle { radius: 1.5 },
        Shape::Rectangle { width: 3.0, height: 4.0 },
        Shape::Triangle { a: 3.0, b: 4.0, c: 5.0 },
    ];
    for s in &shapes {
        println!("{:<10} area {:>7.3}", s.name(), s.area());
    }

    let origin = Point { x: 0.0, y: 0.0 };
    let p = Point { x: 3.0, y: 4.0 };
    println!("distance from origin: {}", origin.distance_to(p));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn right_triangle_area() {
        assert!((Shape::Triangle { a: 3.0, b: 4.0, c: 5.0 }.area() - 6.0).abs() < 1e-9);
    }

    #[test]
    fn rectangle_area() {
        assert_eq!(Shape::Rectangle { width: 2.0, height: 5.0 }.area(), 10.0);
    }

    #[test]
    fn distance_is_pythagorean() {
        assert_eq!(Point { x: 0.0, y: 0.0 }.distance_to(Point { x: 3.0, y: 4.0 }), 5.0);
    }
}
