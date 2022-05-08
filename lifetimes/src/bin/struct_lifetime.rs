/**
 * This annotation means an instance of `ImportantExcerpt` can't outlive
 * the reference it holds in its `part` field.
 */
struct ImportantExcerpt<'a> {
  part: &'a str,
}

/**
 * Similar to generics, lifetimes names for struct fields
 * need to be declared after the `impl` keyword and then used
 * after the struct's name, because those lifetimes are part of
 * the struct's type.
 */
impl<'a> ImportantExcerpt<'a> {
  // 1st elision rule applies, &self lifetime is inferred so no need
  // to explicitly define it.
  // The return value is not a reference, so all good.
  fn level(&self) -> i32 {
    3
  }

  // 1st elision rule applies, each input param gets its own lifetime.
  // 3rd elision rule applies, the return type gets the same lifetime as 
  // that of &self.
  fn announce_and_return_part(&self, announcement: &str) -> &str {
    println!("Attention please: {}", announcement);
    self.part
  }
}

fn main() {
  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find a '.'");
  let i = ImportantExcerpt {
    part: first_sentence
  };
}