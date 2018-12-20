/**
 * Problem 10: Summation of primes
 * The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
 * Find the sum of all the primes below two million.
 */
extern crate euler;
use euler::is_prime;

fn main() {
    let mut primes: Vec<i64> = vec![2, 3, 5];
    let mut sum = 0;
    for i in 2..2_000_000 {
        if is_prime(i, &mut primes) {
            println!("prime {}", i);
            sum += i;
        }
    }
    println!("sum {}", sum);
}
