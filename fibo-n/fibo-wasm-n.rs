use std::time::SystemTime;

fn main() {
    println!("fibo-wasm-n");
    let start = SystemTime::now();
    println!("fibonacci(46) = {}", fibonacci(46));
    match start.elapsed() {
        Ok(elapsed) => {
            println!("elapsed: {}.{:03} sec", elapsed.as_secs(), elapsed.subsec_nanos() / 1000000);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

fn fibonacci(n:i64) -> i64 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
    