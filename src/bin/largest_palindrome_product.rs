/**
 * Problem 4: Largest palindrome product
 * A palindromic number reads the same both ways. The largest palindrome made from the
 * product of two 2-digit numbers is 9009 = 91 × 99.
 * Find the largest palindrome made from the product of two 3-digit numbers.
**/
use std::i32;
use std::f64;

fn count_digits(number: i32) -> i32
{
    f64::floor(f64::log10(number as f64)) as i32 + 1
}

fn nth_digit(number: i32, digit: i32) -> i32
{
    (number / (i32::pow(10, digit as u32))) % 10
}

fn reverse_digits(number: i32) -> i32
{
    let num_digits = count_digits(number);
    let mut reversed: i32 = 0;
    for i in 0..num_digits {
        let digit = nth_digit(number, i);
        let new_place = (num_digits - 1 - i) as u32;
        reversed += digit * i32::pow(10, new_place);
    }
    return reversed
}

fn main()
{
    let mut largest = 1_i32;
    for x in 1..100 {
        for y in 1..100 {
            let product = x * y;
            if product == reverse_digits(product) {
                if product > largest {
                    largest = product;
                    print!("x: {}, y: {}, largest: {}\n", x, y, largest)
                }
            }
        }
    }
}