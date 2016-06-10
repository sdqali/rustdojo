#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(rect: Rectangle) -> f64 {
    let Rectangle {
        p1: Point{x: x1, y: y1},
        p2: Point{x: x2, y: y2}
    } = rect;
    ((x1 - x2) * (y1 - y2)).abs()
}

fn square(point: Point, width: f32) -> Rectangle {
    let Point {x, y} = point;
    Rectangle {
        p1: point,
        p2: Point {
            x: x + width as f64,
            y: y + width as f64,
        }
    }
}

fn main() {
    let rect = Rectangle {
        p1: Point{x: 10.6, y: 34.2},
        p2: Point{x: -67.3, y: 76.21},
    };
    println!("Area is {}", rect_area(rect));
    let corner = Point {x: 10.02, y: 34.2};
    let width = 43.54;
    println!("Square for corner {:?} and width {:?} is {:?}", corner, width,  square(corner, width));
}
