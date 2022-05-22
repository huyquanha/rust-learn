fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}

#[test]
fn iterator_demo() {
    let v1 = vec![1, 2, 3];

    // Calling next() on the iterator changes its internal state,
    // so the iterator needs to be mutable.
    let mut v1_iter = v1.iter();

    // The values we get from next() are immutable references to the
    // values in the vector. If we want to create an iterator that
    // takes ownership of v1 and return owned values, use into_iter()
    // instead of iter(). Similarly if we want to iterate over mutable
    // refernces, call iter_mut() instead of iter().
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

#[test]
fn iterator_adaptor() {
	let v1 = vec![1, 2, 3];

	let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

	assert_eq!(v2, vec![2, 3, 4]);
}

// PartialEq and Debug traits are needed for assert_eq!
// in the test.
#[derive(PartialEq, Debug)]
struct Shoe {
	size: u32,
	style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
	shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn filters_by_size() {
		let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

		let in_my_size = shoes_in_size(shoes, 10);

		assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
	}
}

struct Counter {
	count: u32,
}

impl Counter {
	fn new() -> Counter {
		Counter { count: 0 }
	}
}

impl Iterator for Counter {
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item> {
		if self.count < 5 {
			// this is why we need to borrow self mutably.
			self.count += 1;
			Some(self.count)
		} else {
			None
		}
	}
}

#[test]
fn calling_next_directly() {
	let mut counter = Counter::new();

	assert_eq!(counter.next(), Some(1));
	assert_eq!(counter.next(), Some(2));
	assert_eq!(counter.next(), Some(3));
	assert_eq!(counter.next(), Some(4));
	assert_eq!(counter.next(), Some(5));
	assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
	let sum: u32 = Counter::new()
		// zip returns another iterator that will iterate over 2 other
		// iterators, returning a tuple with one element coming from each iterator.
		// When either iterator runs out of elements and returns None, then
		// the zipped iterator also returns None.	
		.zip(Counter::new().skip(1))
		.map(|(a, b)| a * b)
		.filter(|x| x % 3 == 0)
		.sum();
	assert_eq!(18, sum);
}
