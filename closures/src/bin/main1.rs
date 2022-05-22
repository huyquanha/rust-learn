fn main() {
	let x = 4;

	// Closures can capture the surrounding environment i.e x.
	// There are 3 ways closures can capture the environment:
	// - FnOnce: take ownership of the variable and move them into the
	// closure when it is defined. The "Once" represents that the closure
	// can't take ownership of the same variables more than once, so it can
	// be called only once.
	// - FnMut: can change the environment because it mutable borrows values.
	// - Fn: borrow values from the environment immutably.
	//
	// All closurs implement FnOnce because they can all be called at least once,
	// Closures that don't move the captured variables also implement FnMut, and
	// closures that don't need mutable access to the capture variables also implement
	// Fn. In our example below, equal_to_x has the Fn trait because it borrows
	// x immutably.
	let equal_to_x = |z| z == x;

    println!("can't use x here: {:?}", x);

	let y = 4;

	assert!(equal_to_x(y));

	let z = 5;

	assert!(!equal_to_x(z));

	// To force closure to take ownership, use `move` keyword. A `move` closure
	// can still implement Fn or FnMut, that's because the traits implemented
	// by a closure type are determined by what the closure does with the captured
	// values, not how it captures them. The move keyword only specify the latter.
	// Note that if x implements the Copy trait then using move will cause the value
	// to be copied into the closure insteaad of being moved. We need to use something
	// like a vector to show that the original variable no longer works.
	let x = vec![1, 2, 3];

	let equal_to_x_vector = move |m: Vec<i32>| m == x;

	// This will error out, because x has been moved into the closure.
	// println!("can't use x here: {:?}", x);
}