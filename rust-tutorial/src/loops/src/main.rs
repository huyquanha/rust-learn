fn main() {
    /*
        Definite Loop: loop a predefined number of times.
     */
    // for each i from 0 (inclusive) to 5 (exclusive), print i.
    for i in 0..5 {
        println!("{}", i);
    }

    // count is like the loop index, and variable is the actual value.
    for (count, variable) in (7..10).enumerate() {
        println!("count = {}, variable = {}", count, variable);
    }

    /*
        Indefinite loop
     */
    let mut var = 1;
    let mut found = false;
    // define a while loop
    while !found {
        var += 1;
        //print the variable
        println!("{}", var);
        if var % 2 == 1 {
            found = true;
        }
        println!("Loop runs");
    }

    // Run indefinitely with "loop".
    // let mut var = 1;
    // loop {
    //     var = var + 1;
    //     println!("{}", var);
    // }

    // Using "break".
    for i in 0..10 {
        println!("i:{}", i);
        if i == 5 {
            break;
        }
    }

    let mut i = 1;
    loop {
        println!("i:{}", i);
        if i == 5 {
            break;
        }
        i = i + 1;
    }

    // Using "continue".
    for var in 0..10 {
        if var == 4 {
            println!("I encoutered a continue statement");
            continue;
        }
        println!("var: {}", var);
        println!("I did not encounter continue statement");
    }

    let mut var = 1;
    let mut found = false;
    while !found {
        var = var + 1;
        println!("{}", var);

        if var == 4 {
            println!("I encoutered a continue statement");
            continue;
        }
        println!("I did not encounter continue statement");

        if var == 10 {
            found = true;
        }
    }

    // Nested loops.
    for i in 1..5 { //outer loop
        println!("Multiplication Table of : {}", i);
        for j in 1..5 { // inner loop
            println!("{} * {} = {}", i, j, i * j);
        }
    }

    // Using loop labels to break/continue to a certain loop level.
    // By default, break/continue will take affect on the most-nested loop.
    println!("\nExample using loop label...");
    'outer:for i in 1..5 { //outer loop
        println!("Multiplication Table : {}", i);
        // Throws in a little reverse here. Note that the reversal happens
        // after 1..5 is enumerate i.e 0,1,2,3,4 --> 4,3,2,1,0.
        'inner:for j in (1..5).rev() { // inner loop
            if i == 3 { continue 'outer; } // Continues the loop over `i`.
            if j == 2 { continue 'inner; } // Continues the loop over `j`.
            println!("{} * {} = {}", i, j, i * j);
        }
    }
}
