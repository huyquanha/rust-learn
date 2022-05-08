#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    // This is used to bring Rectangle into scope.
    use super::*;

    fn add_two(number: i32) -> i32 {
        number + 3
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn it_adds_two() {
        // assert_eq! will also print out left and right value
        // in case there's a failure. This and assert_ne! uses
        // == and != to compare 2 values behind the scene, and
        // uses debug formatting to print out values when the assertions
        // fail, so your custom structs/enums must implement PartialEq
        // and Debug trait. These are derivable most of the time
        // so it's jus a matter of adding #[derive(PartialEq, Debug)]
        // on top of your struct/enum definition.
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // You can pass custom error messages after the one required
        // argument in assert!, or 2 required args in assert_eq! or
        // assert_ne! and Rust will use the format! macro to print
        // that out so you can use {} within that.
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    // You could provide an expected message which is a substring
    // of the panic error message you expect to be thrown, to make
    // sure the test does not pass the test for a different panic reason
    // that what you expect.
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // An example of returning Err() instead of panicking when it fails.
    #[test]
    // Writing tests so they return a Result<T, E> enables you 
    // to use the question mark operator in the body of tests, 
    // which can be a convenient way to write tests that 
    // should fail if any operation within them returns an Err variant.
    // You can’t use the #[should_panic] annotation on tests that use Result<T, E>
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
