use std::str::FromStr;
use std::env;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
            .expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    debug_assert!(n != 0 && n != 0);
    
    // Euclid's algorithm
    while m != 0 {
        // if m is not 0, then n % m is the remainder of n divided by m
        if m < n {
            // swap m and n
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }

    n
}