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
    reversed
}

use std::collections::btree_map::BTreeMap;

pub fn print_factorial(factor_map: &BTreeMap<i64, i64>)
{
    for (k, v) in factor_map {
        println!("{} * {} = {}", k, v, k * v);
    }
}

pub fn factorize(factorant: i64) -> BTreeMap<i64, i64>
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


pub fn factorize_with_cache(factorant: i64, cache: &[i64]) -> BTreeMap<i64, i64>
{
    let mut melt = factorant;
    let mut factors = BTreeMap::new();
    let max_factor = (factorant as f64).sqrt().ceil() as i64 + 1;
    // remove composite factors
    for i in cache.iter() {
        while melt % i == 0 {
            *factors.entry(i).or_insert(0) += 1;
            melt /= i;
        }
    }
    // cache exhausted, brute force
    let brute_force = factorize(melt);
    for (key, value) in brute_force.into_iter() {
        *factors.entry(key).or_insert(0) += value;
        cache.push(key);
    }
    factors
}


/* cache is a sorted list of all prime numbers */
fn is_prime(n: i64, cache: &[i64]) -> bool
{
    let factors = factorize_with_cache(n, cache);
    if factors.contains_key(n) {
        return true
    }
    false
}