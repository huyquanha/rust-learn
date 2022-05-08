fn main() {
    let learn_language = "Rust";
    // if construct
    if learn_language == "Rust" {
        println!("You are learning Rust!");
    } else if learn_language == "Java" {
        println!("You are learning Java!");
    }
    else {
        println!("You are learning some other languages!");
    }

    // In Rust, you can also use "if" like a ternary operators in Java/C.
    let res = if learn_language == "Rust" { "You are learning Rust!" } else { "You are learning some other language!" };
    println!("{}", res);

    /*
        If let expressions.
     */
    // "course" is defined as a scrutinee expression.
    let course = ("Rust", "beginner","course");
    // pattern matches with the scrutinee expression
    if let ("Rust", "beginner","course") = course {
        println!("Wrote all values in pattern to be matched with the scrutinee expression");
    } else {
        // do not execute this block
        println!("Value unmatched");
    }

    // If 1st and 2nd value matches, it can guess the 3rd value.
    if let ("Rust", "beginner", c) = course {
        println!("Wrote first two values in pattern to be matched with the scrutinee expression : {}", c);
    }
    else {
        // do not execute this block
        println!("Value unmatched");
    }

    // If 1st value matches, it can guess 2nd and 3rd value.
    if let ("Rust", c, d) = course {
        println!("Wrote one value in pattern to be matched with the scrutinee expression.Guessed values: {}, {}",c,d);
    } else {
        // do not execute this block
        println!("Value unmatched");
    }

    // When the pattern is not matched.
    let course = ("Rust", "beginner");
    // pattern does not match with the scrutinee expression
    if let ("Java", c) = course {
        println!("Course is {}", c);
    } else {
        // execute this block
        println!("Value unmatched");
    }

    // Catch all using "_" pattern. This will be more apparent when we discuss "match".
    // no pattern is define
    if let _ = 10 {
        println!("irrefutable if-let pattern is always executed");
    }

    /*
        Match expressions. Similar to switch statements in Java/C.
     */
    // define a variable
    let x = 5;
    // define match expression
    match x {
        1 => println!("Java"),
        2 => println!("Python"),
        3 => println!("C++"),
        4 => println!("C#"),
        // Rust should be printed.
        5 => println!("Rust"),
        6 => println!("Kotlin"),
        _ => println!("Some other value"),
    };

    // define a variable
    let course = "C++";
    // return value of match expression in a variable.
    let found_course = match course {
        "Rust" => "Rust",
        "Java" => "Java",
        "C++" => "C Plus Plus",
        "C#" => "C Sharp",
        // Note that a catch-all is required if the above matching arms don't cover all
        // the values of the variable being matched. If we comment out the following line,
        // the compiler will error.
        _ => "Unknown Language"
    };
    // Should print "C Plus Plus".
    println!("Course name : {}",found_course);
}
