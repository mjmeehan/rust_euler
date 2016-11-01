/**
 *  Useful functions extracted from Project Euler solutions
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

pub fn reverse_digits(number: i32) -> i32
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

pub fn factorize(factorant:i64) -> Vec<i64>
{
    let mut value = factorant;
    let mut factors = Vec::new();
    let max_factor: i64 = (value as f64).sqrt().round() as i64;
    for i in 2..max_factor {
       if value % i == 0 {
           factors.push(i);
           value /= i;
       }
    }
    factors
}