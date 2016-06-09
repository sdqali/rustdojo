use std::fmt;

#[derive(Debug)]
struct MinMax(Point2, Point2);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2 {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<x:{}, y: {}>", self.y, self.y)
    }
}

fn main() {
    let range =   MinMax(Point2 {x :13.4, y: 65.9}, Point2 {x :48.54, y: 176.98});
    println!("The range is {}", range);
}
