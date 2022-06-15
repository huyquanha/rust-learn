- `&` (reference) is a pointer and borrow the value they point to.
- Smart pointers have additional metadata/capabilities.
	- Reference counting smart pointer: enables you to have multiple owners of data by keeping track of the number of owners and when no owners remain, clean up the data.
	- References only borrow data, but in many cases, smart pointers _own_ the data they point to.
	- `String` and `Vec<T>` are some examples of smart pointers.
	- Smart pointers implement the `Deref` and `Drop` traits.
		- `Deref` trait allows an instance of the smart pointer to behave like a reference so you can write code that works with either references or smart pointers. For example, a method that receives `&str` can work with both `&str` and `String`.
		- `Drop` trait allows you to customise the code that is run when an instance of the smart pointer goes out of scope.

**Box**
- The most straightfoward smart pointer, allowing you to store data on the heap rather than the stack. What remains on stack is the pointer to the heap data. 
- When to use?
	- When you have a type of unknown size at compile time and you want to use a value of that type in a context that requires an exact size.
	- When you have a large amount of data to transfer ownership. Putting them all on the stack can be time consuming to copy around. Instead, storing the data on the heap, and then only copy the pointer data on the stack.
	- When you want to own a value and you care only that it's a type that implements a particular trait than being of a specific type.

```rust
	fn main() {
		let b = Box::new(5);
		// the value is auto-unboxed.
		println!("b = {}", b);
		// When b goes out of scope, both the pointer
		// stored on stack and the data it points to 
		// will be deallocated.
	}
```