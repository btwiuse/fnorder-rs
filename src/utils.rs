//! Utility functions for string manipulation and random word selection.

use crate::words::{ACTIONS, ADJECTIVES, INTROS, NAMES, NOUNS, PLACES, PREPOSITIONS, PRONOUNS};

/// A trait abstraction for random number generation to allow swapping implementations.
pub trait Random {
    /// Returns a random boolean with the given probability (0.0 to 1.0)
    fn random_bool(&mut self, probability: f64) -> bool;

    /// Returns a random value in the given range (inclusive start, exclusive end)
    fn random_range(&mut self, range: std::ops::Range<usize>) -> usize;

    /// Returns a random boolean (equivalent to random_bool(0.5))
    fn random(&mut self) -> bool {
        self.random_bool(0.5)
    }

    /// Returns a random element from a slice, or None if the slice is empty
    fn choose<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T>;

    /// Returns a random action verb
    fn action(&mut self) -> &'static str {
        self.choose(ACTIONS).unwrap()
    }

    /// Returns a random adjective
    fn adjective(&mut self) -> &'static str {
        self.choose(ADJECTIVES).unwrap()
    }

    /// Returns a random introduction phrase
    fn intro(&mut self) -> &'static str {
        self.choose(INTROS).unwrap()
    }

    /// Returns a random name
    fn name(&mut self) -> &'static str {
        self.choose(NAMES).unwrap()
    }

    /// Returns a random noun
    fn noun(&mut self) -> &'static str {
        self.choose(NOUNS).unwrap()
    }

    /// Returns a random place name
    fn place(&mut self) -> &'static str {
        self.choose(PLACES).unwrap()
    }

    /// Returns a random preposition
    fn preposition(&mut self) -> &'static str {
        self.choose(PREPOSITIONS).unwrap()
    }

    /// Returns a random pronoun
    fn pronoun(&mut self) -> &'static str {
        self.choose(PRONOUNS).unwrap()
    }
}

/// Implementation of Random trait for rand::rng()
impl Random for rand::rngs::ThreadRng {
    fn random_bool(&mut self, probability: f64) -> bool {
        rand::Rng::random_bool(self, probability)
    }

    fn random_range(&mut self, range: std::ops::Range<usize>) -> usize {
        rand::Rng::random_range(self, range)
    }

    fn choose<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        use rand::seq::IndexedRandom;
        slice.choose(self)
    }
}

/// Creates a new thread-local random number generator
pub fn rng() -> impl Random {
    rand::rng()
}

/// Capitalizes the first character of a string.
///
/// # Arguments
///
/// * `s` - The string to capitalize
///
/// # Returns
///
/// A new string with the first character converted to uppercase.
/// If the input is empty, returns an empty string.
///
/// # Examples
///
/// ```
/// use fnorder::utils::ucfirst;
///
/// assert_eq!(ucfirst("hello"), "Hello");
/// assert_eq!(ucfirst(""), "");
/// assert_eq!(ucfirst("WORLD"), "WORLD");
/// ```
pub fn ucfirst(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}
