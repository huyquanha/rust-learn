const ID_1: i32 = 4; // define a global constant variable

fn main() {
    let language = "Rust";
    // language = "French"; // this will fails, Rust variables are immutable by default.
    println!("Language: {}", language); // print the variable

    // To make a variable mutable
    let mut programming_language = "Rust";
    println!("Programming Language: {}", programming_language); // print the variable
    programming_language = "Java"; // This works now.
    println!("Programming Language: {}", programming_language); // print the variable

    // Assign multiple variables at once.
    let (course, category) = ("Rust", "beginner");
    println!("This is a {} course in {}.", category, course);

    // Variable scopes.
    let outer_variable = 112;
    { // start of code block
        let inner_variable = 213;
        println!("block variable inner: {}", inner_variable);
        println!("block variable outer: {}", outer_variable);
    } // end of code block
    // println!("inner variable: {}", inner_variable); // use of inner_variable outside scope will fail.

    // Variable shadowing: inner variable has higher priority => outer variable
    // is shadowed.
    { // start of code block
        let inner_variable = 213;
        println!("block variable: {}", inner_variable);
        let outer_variable = 117; // this will override 112.
        println!("block variable outer: {}", outer_variable);
    } // end of code block
    println!("outer variable: {}", outer_variable); // this still gives 112.

    // Constant variables. Differences with let variables include:
    // - const variables can be declare in global scope, but let variables can only be used in local scope.
    // - const variables cannot be mutable, while let variables are immutable by default, but can be made
    // mutable using "mut".
    // - It is mandatory to define an explicit data type for "const" variables, while that can be implicitly
    // inferred for let variables.
    // - const variables cannot be shadowed.
    const ID_2: u32 = 3; // define a local constant variable.
    println!("ID:{}", ID_1); // print the global constant variable
    println!("ID:{}", ID_2); // print the local constant variable


}
