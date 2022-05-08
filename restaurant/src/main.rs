// Run with cargo run --bin restaurant
// The reason we need to specify `--bin restaurant` is because
// of the presence of src/bin directory with some other binary crates
// in there, so we need a name to distinguish this from them.
// "src/main.rs" is the crate root of a binary crate with the same name as the package,
// hence `restaurant`.
fn main() {
  println!("Hello world main");
}