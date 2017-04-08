use std::time::SystemTime;

fn main() {
    println!("nthPrime-wasm");
    let start = SystemTime::now();
    println!("nthPrime(50000) = {}", nth_prime(50000));
    match start.elapsed() {
        Ok(elapsed) => {
            println!("elapsed: {}.{:03} sec", elapsed.as_secs(), elapsed.subsec_nanos() / 1000000);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

fn nth_prime(n:i64) -> i64 {
    let mut cnt = n;
    let mut i = 3;
    let mut primes = vec![2];
    cnt -= 1;
    while cnt > 0 {
        if primes.iter().all(|&x| i % x != 0) {
            primes.push(i);
            cnt -= 1;
        }
        i += 2;
    }
    primes.pop().unwrap_or(-1)
}
    