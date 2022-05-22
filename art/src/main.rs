// This is what it'd have been without the re-exporting with
// `pub use`.
// use art::kinds::PrimaryColor;
// use art::utils::mix;

// This is what it can be with re-exporting.
use art::PrimaryColor;
use art::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}