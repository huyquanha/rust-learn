/**
 * An example of a recursive data type. A List
 * can either be Nil (the base case) or a i32 followed
 * by a smaller List. This is an example of a type
 * where a the size is not known at compile time.
 */
enum List {
	Cons(i32, List),
	Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
