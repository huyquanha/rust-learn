use add_one;
// Even though add_one depends on rand, this will only work
// if rand is explicitly added as dependency of adder.
// The workspace will ensure only a single version of rand
// is used by both packages.
use rand;

fn main() {
    let num = 10;
    println!(
        "Hello, world! {} plus one is {}",
        num,
        add_one::add_one(num)
    );
}
