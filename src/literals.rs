use std::mem::size_of_val;

fn main() {
    let x = 1u8;
    let y = 34;
    let z = 76.54;

    println!("size of 'x' inn bytes: {}", size_of_val(&x));
    println!("size of 'y' inn bytes: {}", size_of_val(&y));
    println!("size of 'z' inn bytes: {}", size_of_val(&z));
}
