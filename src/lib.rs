use rand::seq::IndexedRandom;

pub mod api;
pub mod at_command;
pub mod device_map;
pub mod parser;
pub mod scanner;
pub mod sms;

pub fn get_random_number(numbers: &[String]) -> Option<&String> {
    let mut rng = rand::rng();

    numbers.choose(&mut rng)
}
