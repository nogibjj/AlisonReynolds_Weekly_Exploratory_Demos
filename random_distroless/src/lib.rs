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

// function to flip n coins and return the fraction of heads
pub fn coin_n(n: u32) -> f64 {
    let mut rng = rand::thread_rng();
    let mut count = 0;
    for _ in 0..n {
        let x: bool = rng.gen();
        if x {
            count += 1;
        }
    }
    count as f64 / n as f64
}
