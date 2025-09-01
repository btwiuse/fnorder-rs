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

/// Returns a random action verb from the actions word list.
///
/// # Examples
///
/// ```
/// use fnorder::utils::action;
///
/// let verb = action();
/// println!("The hero {} the dragon", verb);
/// ```
pub fn action() -> &'static str {
    rng().choose(ACTIONS).unwrap()
}

/// Returns a random adjective from the adjectives word list.
///
/// # Examples
///
/// ```
/// use fnorder::utils::adjective;
///
/// let adj = adjective();
/// println!("The {} sword gleams", adj);
/// ```
pub fn adjective() -> &'static str {
    rng().choose(ADJECTIVES).unwrap()
}

/// Returns a random introduction phrase from the intros word list.
///
/// # Examples
///
/// ```
/// use fnorder::utils::intro;
///
/// let opening = intro();
/// println!("{} begins the tale", opening);
/// ```
#[allow(dead_code)]
pub fn intro() -> &'static str {
    rng().choose(INTROS).unwrap()
}

/// Returns a random name from the names word list.
///
/// # Examples
///
/// ```
/// use fnorder::utils::name;
///
/// let character = name();
/// println!("{} draws their weapon", character);
/// ```
pub fn name() -> &'static str {
    rng().choose(NAMES).unwrap()
}

/// Returns a random noun from the nouns word list.
///
/// # Examples
///
/// ```
/// use fnorder::utils::noun;
///
/// let object = noun();
/// println!("The ancient {} holds power", object);
/// ```
pub fn noun() -> &'static str {
    rng().choose(NOUNS).unwrap()
}

/// Returns a random place name from the places word list.
///
/// # Examples
///
/// ```
/// use fnorder::utils::place;
///
/// let location = place();
/// println!("Journey to {}", location);
/// ```
pub fn place() -> &'static str {
    rng().choose(PLACES).unwrap()
}

/// Returns a random preposition from the prepositions word list.
///
/// # Examples
///
/// ```
/// use fnorder::utils::preposition;
///
/// let prep = preposition();
/// println!("Look {} the bridge", prep);
/// ```
pub fn preposition() -> &'static str {
    rng().choose(PREPOSITIONS).unwrap()
}

/// Returns a random pronoun from the pronouns word list.
///
/// # Examples
///
/// ```
/// use fnorder::utils::pronoun;
///
/// let pro = pronoun();
/// println!("Take {} with you", pro);
/// ```
pub fn pronoun() -> &'static str {
    rng().choose(PRONOUNS).unwrap()
}
