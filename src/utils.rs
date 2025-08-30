use rand::seq::IndexedRandom;

use crate::words::{ACTIONS, ADJECTIVES, INTROS, NAMES, NOUNS, PLACES, PREPOSITIONS, PRONOUNS};

pub fn ucfirst(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

pub fn action() -> &'static str {
    ACTIONS.choose(&mut rand::rng()).unwrap()
}

pub fn adjective() -> &'static str {
    ADJECTIVES.choose(&mut rand::rng()).unwrap()
}

#[allow(dead_code)]
pub fn intro() -> &'static str {
    INTROS.choose(&mut rand::rng()).unwrap()
}

pub fn name() -> &'static str {
    NAMES.choose(&mut rand::rng()).unwrap()
}

pub fn noun() -> &'static str {
    NOUNS.choose(&mut rand::rng()).unwrap()
}

pub fn place() -> &'static str {
    PLACES.choose(&mut rand::rng()).unwrap()
}

pub fn preposition() -> &'static str {
    PREPOSITIONS.choose(&mut rand::rng()).unwrap()
}

pub fn pronoun() -> &'static str {
    PRONOUNS.choose(&mut rand::rng()).unwrap()
}
