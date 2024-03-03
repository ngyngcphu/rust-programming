use std::fmt::Debug;

trait Drawable {
    fn bounds(&self) -> Bounds;
}

#[derive(Debug)]
struct Bounds {
    top_left: Point,
    bottom_right: Point,
}

#[derive(Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

struct Square {
    top_left: Point,
    side_length: i64,
}

impl Square {
    fn new(x: i64, y: i64, side_length: i64) -> Self {
        Square {
            top_left: Point { x, y },
            side_length,
        }
    }
}

impl Drawable for Square {
    fn bounds(&self) -> Bounds {
        Bounds {
            top_left: self.top_left.clone(),
            bottom_right: Point {
                x: self.top_left.x + self.side_length,
                y: self.top_left.y - self.side_length,
            },
        }
    }
}

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: i64,
}

impl Circle {
    fn new(x: i64, y: i64, radius: i64) -> Self {
        Circle {
            center: Point { x, y },
            radius,
        }
    }
}

impl Drawable for Circle {
    fn bounds(&self) -> Bounds {
        Bounds {
            top_left: Point {
                x: self.center.x - self.radius,
                y: self.center.y + self.radius,
            },
            bottom_right: Point {
                x: self.center.x + self.radius,
                y: self.center.y - self.radius,
            },
        }
    }
}

struct Container<T>(T);

impl<T: Drawable> Container<T> {
    fn area(&self) -> i64 {
        let bounds = self.0.bounds();
        (bounds.bottom_right.x - bounds.top_left.x) * (bounds.bottom_right.y - bounds.top_left.y)
    }
}

impl<T: Drawable + Debug> Container<T> {
    fn show(&self) {
        println!("{:?} has bounds {:?}", self.0, self.0.bounds());
    }
}

trait Shape: Drawable {
    fn render_in(&self, bounds: Bounds);
    fn render(&self) {
        self.render_in(overlap(SCREEN_BOUNDS, self.bounds()));
    }
}

impl Shape for Square {
    fn render_in(&self, bounds: Bounds) {
        println!("Rendering square within bounds: {:?}", bounds);
    }
}

fn overlap(bounds1: Bounds, bounds2: Bounds) -> Bounds {
    let top_left = Point {
        x: bounds1.top_left.x.max(bounds2.top_left.x),
        y: bounds1.top_left.y.max(bounds2.top_left.y),
    };
    let bottom_right = Point {
        x: bounds1.bottom_right.x.min(bounds2.bottom_right.x),
        y: bounds1.bottom_right.y.min(bounds2.bottom_right.y),
    };

    if top_left.x < bottom_right.x && top_left.y < bottom_right.y {
        Bounds {
            top_left,
            bottom_right,
        }
    } else {
        Bounds {
            top_left: Point { x: 0, y: 0 },
            bottom_right: Point { x: 0, y: 0 },
        }
    }
}

const SCREEN_BOUNDS: Bounds = Bounds {
    top_left: Point { x: 0, y: 0 },
    bottom_right: Point { x: 100, y: 100 },
};

fn main() {
    let square = Square::new(1, 2, 2);
    let draw: &dyn Drawable = &square;
    let shape: &dyn Shape = &square;

    let square = Container(Square::new(1, 2, 2));
    let circle = Container(Circle::new(3, 4, 1));

    println!("area(square) = {}", square.area());
    println!("area(circle) = {}", circle.area());
    circle.show();
}
