//! # Art
//!
//! A library for modeling artistic concepts.

// Re exporting to make it easier for consumers to import stuff
// from our library. They can do use art::PrimaryColor instead
// of art::kinds::PrimaryColor for example.
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
		// just a noop implementation.
        SecondaryColor::Green
    }
}
