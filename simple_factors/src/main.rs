use std::env::args;
use std::process::exit;

fn prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut d: u64 = 2;
    while n > 1 {
        while n % d == 0 {
            factors.push(d);
            n /= d;
        }
        d += 1;
        if d * d > n {
            if n > 1 {
                factors.push(n);
                break;
            }
        }
    }
    factors
}

fn main() {
    let ac: Vec<String> = args().collect();

    if ac.len() < 2 {
        eprintln!("Usage: {:?} <numbers>", ac[0]);
        exit(1);
    }
    for i in 1..ac.len() {
        let n: u64 = ac[i].parse::<u64>().unwrap();
        let factors: Vec<u64> = prime_factors(n);
        print!(
            "{}: {}{}",
            ac[i],
            factors[0],
            factors
                .iter()
                .skip(1)
                .map(|&f| format!(" * {}", f))
                .collect::<String>()
        );
        println!();
    }
}
