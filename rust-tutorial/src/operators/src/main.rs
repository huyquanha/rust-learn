fn main() {
    // Arithmetic operators.
    let a = 4;
    let b = 3;

    println!("Operand 1:{}, Operand 2:{}", a , b);
    println!("Addition:{}", a + b);
    println!("Subtraction:{}", a - b);
    println!("Multiplication:{}", a * b);
    println!("Division:{}", a / b);
    println!("Modulus:{}", a % b);

    // Logical operators. Notice here we can re-declare a and b.
    let a = true;
    let b = false;
    println!("Operand 1:{}, Operand 2:{}", a , b);
    println!("AND:{}", a && b);
    println!("OR:{}", a || b);
    println!("NOT:{}", ! a);

    // Bitwise operators.
    // Left shift: move all bits in operand to the left by the number of places
    // specified in operand 2. New bits are filled with zeros. Shifting a value
    // left by 1 position <=> multiply it by 2, 2 positions <=> multiply it by 4.
    // Right shift: move all bits to the right, new bits are filled with zeros.
    // Shifting right 1 position <=> divide it by 2, 2 positions <=> divide it by 4.
    let a = 5;
    let b = 6;
    println!("Operand 1: {}, Operand 2: {}", a , b);
    println!("AND: {}", a & b);
    println!("OR: {}", a | b);
    println!("XOR: {}", a ^ b);
    println!("NOT a: {}", !a);
    println!("Left shift: {}", a << 2);
    println!("Right shift: {}", a >> 1);

    // Type casting operators.
    let a = 15;
    // This will not work, because Rust disallows an integer division by a float.
    // let b = a / 2.0;
    // This will work, but will round the result down to 7.
    // let b = a / 2;
    // This will work, by first casting a to a float (f64), then division to 2.0 is allowed
    // and produces the correct result 7.5.
    let b = (a as f64) / 2.0;
    println!("b: {}", b);

    /*
    Type casting rules:
    - Integers can be type-casted to floats and vice-versa. i <-> f
    - Integers can be type-casted to String. i -> s and since f -> i => f -> s

    - String (&str) or character cannot be type casted to integer/float.
    - Character cannot be type casted to String or vice versa.
     */

    // Borrowing operators. Basically, each Rust variable lives in some memory
    // location. For example, a statement like "let x = 10" allocates memory location 1000
    // for the variable x, which stores the value 10 in it.
    // Then, borrowing operators like "let a = &x | &mut x" means "a" references "x".
    // NOTE: It's very important to understand that "a" does not store 10 explicitly,
    // it stores the memory address of "x" i.e 1000, while "a" itself lives in a different
    // memory address i.e 2000.
    //
    // There are 2 types of borrowing:
    // - Shared borrowing (operand1 = & operand2): operand1 can read data of another operand2,
    // but that piece of data cannot be altered.
    // - Mutable borrowing (operand1 = &mut operand2): the piece of data that is shared and altered
    // by a single variable (but the data will be in-accessible to other variables at that time).

    let x = 10;
    // Compiler error with "Cannot borrow immutable variable x as mutable".
    // let a = &mut x;

    // So for an immutable variable, you can only borrow it in shared mode.
    // Notice the type of "a" is "&i32", which can be thought of as a "reference"
    // to an i32 integer.
    let a = &x;
    println!("Value of x:{}", x);
    // This will follow the memory address of "x" stored in "a" to get the actual
    // value stored in "x" and print it out.
    println!("Value of a:{}", a);
    // This de-referencing gives the same effect, 10 is printed out.
    println!("Value of *a:{}", *a);
    // Compiler error with "Cannot assigned to immutable borrowed content".
    // *a = 11;

    let mut y = 13;
    // Creates an immutable reference b to a mutable variable y.
    let b = &y;
    println!("Value of `b`:{}", b);
    // Once the variable has been borrowed, it cannot be modified
    // using the old reference.
    // y = 14; // this will error out with: cannot assign to `y` because it is borrowed.
    // y += 1; // error out with the same error.

    // Compiler error with "Cannot assigned to immutable borrowed content".
    // Notice here even though "y" itself is mutable, "b" is on shared borrowing
    // (& instead of &mut), which does not grant b permission to mutate y.
    // *b = 14;

    // Creates b as a mutable reference instead.
    let b = &mut y;
    println!("Value of b:{}", b);
    // This will also error out because the variable's been borrowed so we cannot
    // update it through y.
    // y = 14; // this will error out with: cannot assign to `y` because it is borrowed
    // y += 1; // this will error out with: cannot use `y` because it was mutably borrowed

    // This will work, because b is a mutable reference. You can only create
    // a mutable reference to a mutable variable, so we know we can mutate this.
    // "*" is a de-referencing operator, which basically means: following the memory
    // address stored in "b" to get to "y", then store a different value 11 in "y".
    *b = 11;
    println!("Value of b:{}", b); // should be 11 now.
    println!("Value of y:{}", y); // should also be 11 now.

    // For mutable borrowing, we say the the ownership is moved from y to b. The variable
    // can no longer be mutated through y, you have to use b.
}
