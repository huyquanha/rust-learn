/*
 By default the arguments are immutable, so something like param_1 += 1 is forbidden.
 To make the argument mutable you can prepend the parameter with "mut".
 */
fn my_func(param_1:i32, param_2:i32) {
    println!("The first value passed inside function : {}", param_1);
    println!("The second value passed inside function : {}", param_2);
}

fn main() {
    // invoke a function.
    display_message();
    println!("Function ended");

    // Invoke a function with parameters.
    let value_1 = 1;
    let value_2 = 2;
    my_func(value_1, value_2);
    println!("{}", value_1);

    /*
     Pass by value: the values from the calling function are copied to the paramters in the called
     function at the time the function is called. The called function can change the parameter
     values without affecting the variables in the calling function.
     */
    let n = 4;
    println!("Pass by value...");
    println!("The value of n before function call: {}", n);
    // Even though n itself is immutable, since the value is being copied into another
    // variable before passing in, it can be mutable inside the called function.
    square(n);
    // This should still print out 4, because any changes made to the variable
    // inside the called function won't affect n in the calling function.
    println!("The value of n after function call: {}", n);

    /*
    Pass by reference: when we want the called function to make changes to the parameter that are
    reflectable in the calling function, we pass a reference to the variable to the called function
    instead.
     */
    // Here the same variable will be mutated in squareRef() so we need to declare
    // it as mutable.
    let mut n = 4;
    println!("Pass by reference...");
    println!("The value of n before function call: {}", n);
    // mutable borrowing n.
    square_ref(&mut n);
    // Should be 16.
    println!("The value of n after function call: {}", n);

    // Functions with single return value.
    println!("Functions with return value...");
    let  n = 4;
    println!("The value of n before function call: {}", n);
    println!("Output: {}", square_return(n));

    // Functions with multiple return values, in a tuple.
    let length = 4;
    let width = 3;
    println!("Rectangle length:{}", length);
    println!("Rectangle width:{}", width);
    let (area, perimeter) = calc_area_perimeter(length, width);
    println!("Area: {}, Perimeter: {}", area, perimeter);

    // Arrays can be passed to functions by value or reference.
    let arr = [1,2,3,4];
    println!("Passing array by value to function...");
    // Here arr is passed by value to the function, so a copy of the entire
    // array is created before being passed in. Hence, any changes to any elements
    // in the array won't be reflected
    modify_my_array(arr);
    // The array should still be the same.
    println!("Array after function: {:?}", arr);

    println!("Passing array by reference to function...");
    // We need to define the array as mutable, otherwise we cannot
    // mutably borrow a reference to it.
    let mut arr = [1,2,3,4];
    modify_my_array_ref(&mut arr);
    println!("Array after function: {:?}", arr);

    // Returns an array from function.
    let arr = [1, 2, 3, 4];
    println!("Array in Driver Function : {:?}", arr);
    println!("Array after Function Call : {:?}", modify_my_array_ret(arr));

    // Recursion.
    let n = 4;
    let fact = factorial(n);
    println!("Factorial of {} is {}", n, fact);
}

fn display_message() {
    println!("Hi, this is my user-defined function");
}

fn square(mut n:i32) {
    n = n * n;
    println!("The value of n inside function: {}", n);
}

fn square_ref(n:&mut i32) {
    // de-reference n to get the value, times with itself and
    // store it back to n.
    *n = *n * *n;
    println!("The value of n inside function: {}", n);
}

// This function returns an i32 value.
fn square_return(n: i32) -> i32 {
    println!("The value of n inside function : {}", n);
    let m = n * n;
    // We could write "return m;" here but even just m would work. Rust
    // compiler will infer this is the return value based on the return type.
    m
}

fn calc_area_perimeter(x: i32, y: i32) -> (i32, i32) {
    return (x * y, 2 * (x + y));
}

// Without mut, array elements cannot be updated.
fn modify_my_array(mut arr:[i32;4]) {
    arr[2] = 8;
    arr[3] = 9;
    println!("Array in my Function: {:?}", arr);
}

fn modify_my_array_ref(arr: &mut [i32; 4]) {
    arr[2] = 8;
    arr[3] = 9;
    println!("Array in my Function : {:?}", arr);
}

fn modify_my_array_ret(mut arr: [i32; 4]) -> [i32;4] {
    arr[2] = 8;
    arr[3] = 9;
    arr
}

fn factorial(n: i64) -> i64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
