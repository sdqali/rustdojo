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

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    let range =   MinMax(Point2 {x :13.4, y: 65.9}, Point2 {x :48.54, y: 176.98});
    println!("The range is {}", range);

    let complex = Complex{real: 13.87, imag: 987.04};
    println!("Complex number display: {}", complex);
    println!("Complex number debug: {:?}", complex);
}
