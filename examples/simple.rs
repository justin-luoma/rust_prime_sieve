use prime_sieve::primes;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let primes = primes(1_000_000);

    println!("primes: {:?}", primes);
    println!("elapsed: {}ms", start.elapsed().as_millis());
}
