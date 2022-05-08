/**
 * We need Copy trait because otherwise if T is a move type
 * let mut largest = list[0] would fail because list is a
 * borrowing reference, it does not own the slice so the ownership
 * cannot be moved to largest.
 */
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];

  for &item in list {
      if item > largest {
          largest = item;
      }
  }

  largest
}

fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
  // by cloning, we avoid the ownership issue.
  let mut largest = list[0].clone();

  // We have to use item here instead of &item because
  // &item means item is of type T, which requires moving
  // the list elmenent out of a reference into item, which
  // is not possible (because "list" does not own the element),
  // it's only a shared reference.
  for item in list {
      if *item > largest {
          largest = item.clone();
      }
  }

  largest
}

fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];

  for item in list {
    if *item > *largest {
      largest = item;
    }
  }

  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest_ref(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];

  let result = largest_ref(&char_list);
  println!("The largest char is {}", result);
}