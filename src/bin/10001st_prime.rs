/**
 * By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is
 * 13. What is the 10 001st prime number?
 **/
extern crate euler;

fn is_prime(n: i64, cache: &[i64]) -> bool {
    for prime in cache {
        if n % prime == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let mut cache = vec![2, 3, 5, 7, 11, 13];
    let mut i = 14;
    while cache.len() < 10002 {
        if is_prime(i, &cache) {
            println!("found prime {}", i);
            cache.push(i);
        }
        i += 1;
    }
    println! {"{}", cache[10000]};
}
