fn main() {
    // println! is a macro.
    println!("Hello, world!");
    println!("Number: {}", 1);
    println!("Enhance your coding skills from {0} {1} courses. {0} courses are very {1}", "Educative", "interactive");
    println!("{company} provides {kind} courses", company = "Educative", kind = "interactive");
    // Use :b (binary), :x (hex), :o (octal) to convert numbers.
    println!("Number : 10 \nBinary:{0:b} Hexadecimal:{0:x} Octal:{0:o}", 10);
    // Display multiple values using a single placeholder with debug trait :?.
    println!("{:?}", ("This is a Rust Course", 101));

    // print! is similar, except that it doesn't append newlines.
    print!("Rust Programming");
    print!(" Course");

    // eprint! prints as an error. There's also eprintln!
    eprint!("Rust Programming");
    eprint!(" Course");
}
