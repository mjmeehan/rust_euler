/**
 * Problem 6: Sum square difference
 * The sum of the squares of the first ten natural numbers is, 1^2 + 2^2 + ... + 10^2 = 385
 * The square of the sum of the first ten natural numbers is, (1 + 2 + ... + 10)^2 = 55^2 = 3025
 * Hence the difference between the sum of the squares of the first ten natural numbers and the
 * square of the sum is 3025 − 385 = 2640.
 * Find the difference between the sum of the squares of the first one hundred natural
 * numbers and the square of the sum.
**/

extern crate euler;

use std::i32;

fn square_of_sums(n: i32) -> i32
{
    i32::pow(n * (n + 1) / 2, 2)
}

fn sum_of_squares(n: i32) -> i32
{
    n * (n + 1) * (2 * n + 1) / 6
}

fn main()
{
    let n = 20;
    let a = sum_of_squares(n);
    let b = square_of_sums(n);
    print!("{}\n", b-a);
}
