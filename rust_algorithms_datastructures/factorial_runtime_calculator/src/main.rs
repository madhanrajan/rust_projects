use std::time::{Duration, Instant};

fn factorial(n: i32) -> i32{

    if n <= 1 {
        return 1;
    }
    n * factorial(n-1)
}

fn factorial_alt(n: i32, r: i32) -> i32 {

    if n <= 1 {
        return 1;
    }

    factorial_alt(n-1, r*n)

}

fn main() {
    
    let start = Instant::now();
    let _ = factorial(10);
    let duration = start.elapsed();

    let start_alt = Instant::now();
    let _ = factorial(10);
    let duration_alt = start.elapsed();

    println!("Time elapsed on function 1 is {:?}", duration);
    println!("Time elapsed on function 2 is {:?}", duration_alt);

}
