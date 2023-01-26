// function to generate a random boolean output
use rand::prelude::*;

pub fn coin() -> String {
    let mut rng = rand::thread_rng();
    let x: bool = rng.gen();
    if x {
        "Heads".to_string()
    } else {
        "Tails".to_string()
    }
}
