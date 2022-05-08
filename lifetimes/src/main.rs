fn main() {
    // Wrong. x does not live long enough
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }

    // The right version.
    let x = 5;
    let r = &x;

    println!("r: {}", r);
}
