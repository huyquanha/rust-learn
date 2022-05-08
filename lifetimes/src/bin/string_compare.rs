fn main() {
  // In this example, the returned reference in `result` is expected
  // to have the same lifetime as the smaller lifetimes between `string1`
  // and `string2`, which happens to have the same lifetime (till end of main()).
  // The borrow checker checks usages of `result` and see that they conforms
  // to this lifetime (until end of main()) so it approves this code.
  let string1 = String::from("abcd");
  let string2 = "xyz";

  let result = longest(string1.as_str(), string2);
  println!("The longest string is {}", result);

  // Below is a slightly more complicated example where `string1` and `string2`
  // have different lifetimes. `string1` lifetime is till end of outer scope,
  // while `string2` lifetime is till end of inner scope. As per `longest` function
  // signature, the returned reference's lifetime should be the same as the smaller
  // lifetime between x and y => result's lifetime is also till end of inner scope.
  // The borrow checker then checks usages of `result` and see that it's indeed
  // last used in the println! statement in the inner scope => the lifetime is valid
  // and the borrow checker approves this code.
  let string1 = String::from("abcd");

  {
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
  }

  // Another example. In this example, string2 has a smaller lifetime, and
  // result will get the same lifetime (till the end of inner scope). However
  // the borrow check sees a usage of `result` after that (in println! in the outer scope)
  // and will error out because that usage conflicts with the lifetime result is supposed
  // to have. As humans when we look at this code we know result will be string1, so
  // even if string2 goes out of scope it does not matter, but the computer can't see
  // it that way and it will complain anyway.
  // let string1 = String::from("abcd");
  // let result;
  // {
  //   // Notice how we use a heap-allocated String here instead of just 
  //   // `lat string2 = "xyz"`. The reason for this is a string literal
  //   // have 'static lifetime. Since the string literal is hard-coded in the program
  //   // binrary, it will always be available. By using a heap-allocated string
  //   // with String::from() instead, string2 will go out of scope at the end
  //   // of the inner scope and hence has a shorter lifetime than string1.
  //   let string2 = String::from("xyz");
  //   result = longest(string1.as_str(), string2.as_str());
  // }
  // println!("The longest string is {}", result);
}

/**
 * This function will fail, because the borrow checker doesn't
 * know the return value references x or y. The borrow checker
 * needs this information so it can perform static analysis to make sure
 * the return value's lifetime is valid while either x or y is valid.
 */
// fn longest_without_generic_lifetime_parameter(x: &str, y: &str) -> &str {
//   if x.len() > y.len() {
//     x
//   } else {
//     y
//   }
// }

/**
 * The correct version, using generic lifetime parameter 'a,
 * and lifetime annotation &'a str.
 * The constraint we want to express here is the lifetime of both parameters
 * x and y, and the lifetime of the returned reference are related such that
 * the returned reference will be valid as long as both the paramters are
 * (because they all have the same lifetime 'a).
 * 
 * In practice, this means that the lifetime of the reference returned by longest()
 * is the same as the smaller of the lifetimes of the references passed in (x and y).
 * With this knowledge, the borrow checker can perform static analysis to make sure
 * the returned reference is used correctly.
 * 
 * Remember, lifetime annotations don't change the lifetime of any values passed in or returned,
 * it just helps the borrow checker reject any values that don't adhere to these constraints.
 */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
