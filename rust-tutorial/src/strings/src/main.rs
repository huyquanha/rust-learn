/**
It's confusing having to remember which methods can be called on both a String object
and a string literal, and which methods can only be called on String objects.

As a rule of thumb, any methods that grow the string can only be called on String objects,
including
- push()
- push_str()
- "+"
- capacity() (only valid in the context of String objects).

All other methods can be used inter-changeably: len(), contain(), replace(), trim(), [a..b] (slicing)
**/
fn main() {
    /*
    String are of 2 types: &str and String.
     */

    // String literal (&str) are
    // - primitive type
    // - immutable
    // - fixed-length string stored somewhere in memory
    // - value of string is known at compile time.
    let language: &str = "Rust";
    // print the String literal
    println!("String literal: {}", language);
    // print the length of the String literal
    println!("Length of the string literal: {}", language.len());

    // String objects.
    // - encoded as a UTF-8 sequence.
    // - heap-allocated data structure => dynamic
    // - the size can be modified, unlike string literals.
    // - not null-terminated.
    // - encode string values that are given at runtime.

    // Create an empty string.
    let mut language = String::new();
    // print the String object
    println!("This is an empty string{}.", language);
    // print the length of an empty String  object
    println!("This is a length of my empty string {}.", language.len());

    // Adds a new character to this string.
    language.push('a');
    println!("This is no longer an empty string: {}", language);
    // print the length of the string.
    println!("This is the String length: {}.", language.len());

    // Creates from a string literal.
    let language = "Rust";
    let mut S_language = language.to_string();
    S_language.push('s');
    println!("S_language is now: {}", S_language);

    // Creates directly from string literal.
    let language = String::from("Rust");
    println!("language: {}", language);

    // String object normally allocates more bytes what's needed for the actual string stored inside.
    // This is called the capacity, measured in bytes.
    // define a growable string variable
    let course = String::from("Rust");
    println!("This is a beginner course in {}.", course);
    // capacity in bytes
    println!("Capacity: {}.", course.capacity());

    // Check if one string contains another.
    let str = String::from("Rust programming");
    println!("Contain: {}", str.contains("Rust"));

    // Verify that contain() can be called on string literals.
    let str = "Rust programming";
    println!("String literal contain: {}", str.contains("Rust"));

    // Replace. Does not modify the existing string but create
    // a new one.
    let str = String::from("Rust Programming");
    let replace_from = "Programming";
    let replace_to = "Language";
    // find if string contains a substring
    let result = str.replace(replace_from, replace_to);
    println!("{} now becomes {}.", str, result);
    println!("str is still: {}", str);

    // Verify that replace() can be called on string literals.
    let str = "Rust Programming";
    let result = str.replace(replace_from, replace_to);
    println!("{} now becomes {}.", str, result);

    // Trim. Does not modify the existing string but create a new one.
    let string = "   Rust     Programming     ".to_string();
    let trim_string = string.trim();
    // get characters at 5,6,7,8,9,10 and 11 indexes
    println!("Trimmed_string : {}", trim_string);
    println!("string is still: {}", string);

    // Verify that trim() can also be called on string literals.
    let string = "   Rust     Programming     ";
    let trim_string = string.trim();
    // get characters at 5,6,7,8,9,10 and 11 indexes
    println!("Trimmed_string : {}", trim_string);

    // Tokenizing a string.
    // Split on whitespace.
    for found in string.split_whitespace() {
        println!("{}", found);
    }

    // Split on custom character.
    let string = "Rust, programming, love";
    for found in string.split(", ") {
        println!("{}", found);
    }

    // Iterate over each character.
    for found in string.chars() {
        println!("{}", found);
    }

    /*
    NOTE: All methods below are specific to only String objects,
    not string literals.
     */

    // Push a character to a String. The variable has to be mutable,
    // otherwise you won't be able to push.
    let mut course = String::from("Rus");
    course.push('t');
    println!("This is a beginner course in {}.", course);

    // You can also push a String to another String with push_str().
    // define a string object
    let mut course = String::from("Rust");
    // push a string
    course.push_str(" Programming");
    println!("This is a beginner course in {}.", course);

    // Or, using the "+" concatenator.
    let course = "Rust".to_string();
    let course_type = " beginner course".to_string();
    // The "+" expect a &str on the right hand side operand, so we have
    // to add &.
    let result = course + &course_type;
    // Expects "Rust beginner course".
    println!("{}", result);

    // Alternatively, we could define course_type as a string literal.
    let course_type = " beginner course";
    // let result = course + course_type; // this will not work because ownership of the string
    // has been moved to "result" above, so "course" becomes invalid and cannot be used
    // to access the string anymore.

    // let result = &course + &course_type; // this will not work either because "+" cannot
    // be used to concatenate 2 "&str" strings. It requires ownership of the string on the left.

    // This will work because "result" has ownership over the string.
    let result = result + course_type;
    // Expects "Rust beginner course beginner course".
    println!("{}", result);

    // Another way to add multiple Strings together is using format! macro.
    // This will create a new string instead of printing it out like println! macro.
    let course = "Rust".to_string();
    let _course_type = "beginner course".to_string();
    // default format macro
    let result = format!("{} {}", course, _course_type);
    // Expects "Rust beginner course".
    println!("{}", result);

    // passing value in the placeholder in the format macro.
    // Expects "beginner course Rust".
    let result = format!("{1} {0}", course,_course_type);
    println!("{}", result);

    // Slicing a String. This can be done on both a String object
    // and a string literal.
    let string = "Rust Programming".to_string();
    let slice = &string[5..12];
    // get characters at 5,6,7,8,9,10 and 11 indexes
    println!("Slice: {}", slice);

    let string = "Rust Programming";
    let slice = &string[5..12];
    // get characters at 5,6,7,8,9,10 and 11 indexes
    println!("Slice: {}", slice);

    /*
    Passing String objects/literals to functions.
     */
    // String literals can be passed into the function and still
    // usable after that in the calling function.
    let course: &str = "Rust Programming";
    display_course_name(course);
    println!("{}",course); // string literal is used after the function call

    // String objects OTOH cannot be used after being passed into function.
    let mut course:String = String::from("Rust Programming");
    // move occurs because course is of type String which does not implement Copy trait,
    // hence, the original course variable is no longer valid.
    display_course_name_obj(course);
    // course.push('a'); // this will error out because ownership is moved.
}

fn display_course_name(my_course: &str){
    println!("Course : {}", my_course);
}

fn display_course_name_obj(my_course:String){
    println!("Course : {}", my_course);
}