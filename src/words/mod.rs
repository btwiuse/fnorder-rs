//! Word lists for generating fantasy orders and quests.
//!
//! This module contains various categories of words used in the sentence generation:
//! actions, adjectives, names, nouns, places, prepositions, and pronouns.

pub mod actions;
pub mod adjectives;
pub mod intros;
pub mod names;
pub mod nouns;
pub mod places;
pub mod prepositions;
pub mod pronouns;

pub use actions::ACTIONS;
pub use adjectives::ADJECTIVES;
pub use intros::INTROS;
pub use names::NAMES;
pub use nouns::NOUNS;
pub use places::PLACES;
pub use prepositions::PREPOSITIONS;
pub use pronouns::PRONOUNS;
