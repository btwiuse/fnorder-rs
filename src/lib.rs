use rand::Rng;

pub mod utils;
pub mod words;

use utils::{action, adjective, name, noun, place, preposition, pronoun, ucfirst};

pub fn fnorder() -> String {
    let mut rng = rand::rng();
    let mut msg = String::new();

    match rng.random_range(0..14) {
        0 => {
            msg.push_str("The ");
            if rng.random_bool(0.5) {
                msg.push_str(adjective());
                msg.push(' ');
            }
            msg.push_str(noun());
            if rng.random_bool(0.2) {
                msg.push_str(" in ");
                msg.push_str(place());
            }
            msg.push_str(" is ");
            msg.push_str(adjective());
        }
        1 => {
            msg.push_str(name());
            msg.push(' ');
            msg.push_str(action());
            msg.push_str(" the ");
            msg.push_str(adjective());
            msg.push(' ');
            msg.push_str(noun());
            msg.push_str(" and the ");
            msg.push_str(adjective());
            msg.push(' ');
            msg.push_str(noun());
        }
        2 => {
            msg.push_str("The ");
            msg.push_str(noun());
            msg.push_str(" from ");
            msg.push_str(place());
            msg.push_str(" will go to ");
            msg.push_str(place());
        }
        3 => {
            msg.push_str(name());
            msg.push_str(" must take the ");
            msg.push_str(adjective());
            msg.push(' ');
            msg.push_str(noun());
            msg.push_str(" from ");
            msg.push_str(place());
        }
        4 => {
            msg.push_str(place());
            msg.push_str(" is ");
            msg.push_str(adjective());
            msg.push_str(" and the ");
            msg.push_str(noun());
            msg.push_str(" is ");
            msg.push_str(adjective());
        }
        5 => {
            msg.push_str(name());
            msg.push(' ');
            msg.push_str(preposition());
            msg.push(' ');
            msg.push_str(place());
            msg.push_str(" for the ");
            msg.push_str(adjective());
            msg.push(' ');
            msg.push_str(noun());
        }
        6 => {
            msg.push_str("The ");
            if rng.random() {
                msg.push_str(adjective());
                msg.push(' ');
            }
            msg.push_str(noun());
            msg.push(' ');
            msg.push_str(action());
            msg.push_str(" the ");
            msg.push_str(adjective());
            msg.push(' ');
            msg.push_str(noun());
            if rng.random_bool(0.2) {
                msg.push_str(" in ");
                msg.push_str(place());
            }
        }
        7 => {
            msg.push_str(name());
            msg.push(' ');
            msg.push_str(preposition());
            msg.push(' ');
            msg.push_str(place());
            msg.push_str(" and ");
            msg.push_str(action());
            msg.push_str(" the ");
            msg.push_str(noun());
        }
        8 => {
            msg.push_str(name());
            msg.push_str(" takes ");
            msg.push_str(pronoun());
            msg.push(' ');
            if rng.random() {
                msg.push_str(adjective());
                msg.push(' ');
            }
            msg.push_str(noun());
            msg.push_str(" and ");
            msg.push_str(preposition());
            msg.push(' ');
            msg.push_str(place());
        }
        9 => {
            msg.push_str(name());
            msg.push(' ');
            msg.push_str(action());
            msg.push_str(" the ");
            if rng.random() {
                msg.push_str(adjective());
                msg.push(' ');
            }
            msg.push_str(noun());
        }
        10 => {
            msg.push_str(name());
            msg.push(' ');
            msg.push_str(action());
            msg.push(' ');
            msg.push_str(name());
            msg.push_str(" and ");
            msg.push_str(pronoun());
            msg.push(' ');
            if rng.random() {
                msg.push_str(adjective());
                msg.push(' ');
            }
            msg.push_str(noun());
        }
        11 => {
            msg.push_str(name());
            msg.push_str(" is the ");
            if rng.random() {
                msg.push_str(adjective());
                msg.push(' ');
            }
            msg.push_str(noun());
            msg.push_str(": ");
            msg.push_str(name());
            msg.push(' ');
            msg.push_str(preposition());
            msg.push(' ');
            msg.push_str(place());
        }
        12 => {
            msg.push_str("You must meet ");
            msg.push_str(name());
            msg.push_str(" at ");
            msg.push_str(place());
            msg.push_str(" and get the ");
            if rng.random() {
                msg.push_str(adjective());
                msg.push(' ');
            }
            msg.push_str(noun());
        }
        13 => {
            msg.push_str("A ");
            msg.push_str(noun());
            msg.push_str(" from ");
            msg.push_str(place());
            msg.push(' ');
            msg.push_str(action());
            msg.push_str(" the ");
            if rng.random() {
                msg.push_str(adjective());
                msg.push(' ');
            }
            if rng.random_bool(0.2) {
                msg.push_str(adjective());
                msg.push(' ');
            }
            msg.push_str(noun());
        }
        _ => unreachable!(),
    }
    format!("{}.", ucfirst(&msg))
}
