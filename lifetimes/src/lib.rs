use std::fmt::Display;

/**
 * This function does not need lifetime annotations because
 * it only has one input paramter, so the returned reference's lifetime
 * is inferred to be the same as that of the input's lifetime. This is called
 * "lifetime elision rules".
 * - Lifetimes on function or method paramters are called input lfetimes, and
 * on return values are called output lifetimes.
 * - 3 rules:
 * 1. Each function/method parameter that is a reference gets its own lifetime
 * parameter implicitly i.e x gets 'a and y gets 'b.
 * 2. If there is exactly one input lifetime parameter, that lifetime is assigned
 * to all output lifetime paramters. This is what makes first_word() works without
 * explicit lifetime annotations.
 * 3. If there are multiple input lifetime paramters, but one of them is &self or
 * &mut self, the lifetime of `self` is assigned to all output lifetime paramters.
 * This 3rd rule makes methods much nicer to read and write because fewer symbols
 * are necessary.
 * 
 * After all these rules if the compiler still can't figure out the return type's
 * lifetime, it will raise an error and programmers have to jump in and make that
 * explicit.
 */
fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}

/**
 * A function that uses generic, trait bounds and lifetimes altogether.
 * - Because lifetimes are a type of generic, 'a is declared together
 * with T in the angle brackets.
 * - T needs to be printable (used in println!) hence it needs to implement
 * Display trait, hence the where clause
 */
fn longest_with_an_announcement<'a, T>(
  x: &'a str,
  y: &'a str,
  ann: T,
) -> &'a str
where
  T: Display,
{
  println!("Announcement! {}", ann);
  if x.len() > y.len() {
      x
  } else {
      y
  }
}