use std::{thread, time::Duration, collections::HashMap};

struct Cacher<T>
where
    T: Fn(u32) -> u32, 
{
	calculation: T,
	value_map: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
	T: Fn(u32) -> u32,
{
	fn new(calculation: T) -> Cacher<T> {
		Cacher { calculation, value_map: HashMap::new() }
	}

	fn value(&mut self, arg: u32) -> u32 {

		match self.value_map.get(&arg) {
			Some(v) => *v,
			None => {
				let v = (self.calculation)(arg);
				self.value_map.insert(arg, v);
				v
			}
		}
	}
}	

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
		println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
	});

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

// This test should pass now that we use a map to store values.
#[test]
fn call_with_different_values() {
	let mut c = Cacher::new(|a| a);

	let v1 = c.value(1);
	let v2 = c.value(2);

	assert_eq!(v2, 2);
}
