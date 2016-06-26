pub fn inference() {
    let elem = 8f32;
    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}", vec);
}

fn main() {
    inference();
}

#[test]
fn test_inference() {
    inference();
}
