fn main() {
    /*
        SCALAR TYPES
     */
    // Rust variables are statically typed (known in compile time), however
    // unlike C or Java (<11), Rust can infer the types.
    let language = "English"; // infer type string.
    // You can also be explicit about the types.
    let number: i32 = 14;

    // Integers have 2 subtypes:
    // - Fixed size types. letter integer
    //      - letter: u(unsigned) or i(signed).
    //      - integer: 8,16,32,64 denotes the number of bits.
    //      - eg: i16 is 16-bit signed integer, or u32 is 32-bit unsigned integer.
    let a: i32 = 24;
    let b: u64 = 23;
    println!("a: {}", a);
    println!("a in binary: {:b}", a);
    println!("b: {}", b);
    println!("b in binary: {:b}", b);

    // - Variable size types: The particular size depends on the underlying machine architecture.
    //      - isize: The pointer-sized signed integer type i.e on a 32-bit target, this is 4 bytes, or 8 bytes on 64-bit target.
    //      - usize: The pointer-sized unsigned integer type i.e on a 32-bit target, this is 4 bytes, or 8 bytes on 64-bit target.
    let c: usize = 26;
    let d: isize = 29;
    println!("c: {}", c);
    println!("c in binary: {:b}", c);
    println!("d: {}", d);
    println!("d in binary: {:b}", d);

    // Floating point. Consists of f32 (single-precision) and f64 (double precision).
    let f1: f32 = 32.9;
    let f2: f64 = 6789.89;
    println!("f1: {}", f1);
    println!("f2: {}", f2);

    // Booleans.
    let a = true;
    let b: bool = false;
    let c = 10 > 2;
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c); // expects true.

    // Character values are enclosed in single quotes.
    let my_char: char = 'a';
    println!("My char: {}", my_char);

    // String is enclosed in double quotes.
    let my_string: &str = "Rust";
    println!("My favorite language: {}", my_string);

    /*
        COMPOUND TYPES
     */
    // Arrays.
    // Define an immutable array of size 4, comprising of i32 integers.
    let arr: [i32; 4] = [1, 2, 3, 4];
    println!("The first value of array is {}", arr[0]);
    // arr = [2,3,4,5]; // this will error, you can't reassign an immutable array.
    // arr[0] = 2; // this will also error, you can't update any index of an immutable array.
    // let arr = [2,3,4,5]; // this will work because you don't re-assign, you re-define arr.

    let mut arr: [i32; 4] = [1, 2, 3, 4];
    arr = [2,3,4,5]; // this is ok because arr is mutable.
    arr[0] = 3; // this is also ok because arr is mutable.

    // Define another array of size 4 with all 0s.
    let arr1 = [0 ; 4];
    println!("The first value of array is {}", arr1[0]);
    // Define a mutable array.
    let mut mutable_arr:[i32;4] = [1, 2, 3, 4];
    mutable_arr[1] = 9;
    println!("The value of array at index 1: {}", mutable_arr[1]);

    // Using debug trait to print an array.
    println!("Array: {:?}", arr);

    // Get the length of an array.
    println!("Array length: {}", arr.len());

    // Slicing an array. Starting index is inclusive and ending index is exclusive.
    let slice_arr:&[i32] = &arr;
    let slice_arr1:&[i32] = &arr[0..2];
    println!("Slice of an array: {:?}", slice_arr);
    println!("Slice of an array: {:?}", slice_arr1);

    // Tuples: a sequence of elements with different types.
    let my_tuple = ("Value1", 'c', 1);
    // Or you can be explicit.
    let my_tuple1:(&str, char, i32) = ("Value1", 'c', 1);

    // Unlike arrays which access values using [], tuples access values using dot (.)
    let person_data = ("Alex", 48, "35kg", "6ft");
    println!("The value of the tuple at index 0 and index 1 are {} and {}", person_data.0, person_data.1);

    // Or, you can destructure a tuple.
    let (w, x, y, z) = person_data;
    println!("Name : {}", w);
    println!("Age : {}", x);
    println!("Weight : {}", y);
    println!("Height : {}", z);

    // And to make a tuple mutable...
    let mut mutable_person_data = ("Alex", 48, "35kg", "6ft");
    mutable_person_data.0 = "John";
    println!("The person name is {}", mutable_person_data.0);

    // Similar to array, the whole tuple can be printed with debug trait.
    println!("Tuple person_data is: {:?}", person_data)
}
