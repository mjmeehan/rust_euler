/** 
 * Problem 10: Summation of primes
 * The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
 * Find the sum of all the primes below two million.
 */
extern crate euler;
use euler::is_prime;

fn main()
{
    let mut primes = Vec::new();
    let mut sum = 0;
    primes.push(2).push(3).push(5);
    for i in 2..10 {
        if is_prime(i, primes) {
            sum += i;
        }
    }
    println!("{}", sum);
}


