//! Utility functions for string manipulation and random word selection.

use rand::seq::IndexedRandom;

use crate::words::{ACTIONS, ADJECTIVES, INTROS, NAMES, NOUNS, PLACES, PREPOSITIONS, PRONOUNS};

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
    ACTIONS.choose(&mut rand::rng()).unwrap()
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
    ADJECTIVES.choose(&mut rand::rng()).unwrap()
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
    INTROS.choose(&mut rand::rng()).unwrap()
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
    NAMES.choose(&mut rand::rng()).unwrap()
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
    NOUNS.choose(&mut rand::rng()).unwrap()
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
    PLACES.choose(&mut rand::rng()).unwrap()
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
    PREPOSITIONS.choose(&mut rand::rng()).unwrap()
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
    PRONOUNS.choose(&mut rand::rng()).unwrap()
}
