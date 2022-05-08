/**
 * derive is a way to let Rust automatically implements
 * a trait for you, instead of having to implement it yourself.
 */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
