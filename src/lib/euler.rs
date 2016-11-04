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

use std::collections::btree_map::BTreeMap;

pub fn print_factorial(factor_map: &BTreeMap<i64, i64>) {
    for (k, v) in factor_map {
        print!("{} * {} = {} \n", k, v, k * v);
    }
}

pub fn factorize(factorant:i64) -> BTreeMap<i64, i64>
{
    let mut value = factorant;
    let mut factors = BTreeMap::new();
    let max_factor = (value as f64).sqrt().ceil() as i64 + 1;
    // remove composite factors
    for i in 2..max_factor {
       while value % i == 0 {
           *factors.entry(i).or_insert(0) += 1;
           value /= i;
       }
    }
    // number is prime
    if value > 1 {
        factors.insert(value, 1);
    }
    factors
}