/**
 *  2520 is the smallest number that can be divided by each of the numbers
 *  from 1 to 10 without any remainder.
 *  What is the smallest positive number that is evenly divisible by all of
 *  the numbers from 1 to 20?
**/
extern crate euler;
use std::collections::btree_map::BTreeMap;
use std::cmp;

fn main()
{
    let mut grand_factor = BTreeMap::new();
    for divisor in 1..21 {
        println!("divisor: {}", divisor);
        let factors = euler::factorize(divisor);
        for (factor, weight) in factors {
            println!("{} * {}", factor, weight);
            // need to set each entry to the max weight
            // grand factor may be uninitialized
            let value = grand_factor.entry(factor).or_insert(weight);
            *value = cmp::max(*value, weight);
            println!("{}", value);
        }
    }
    let mut grand_total = 1_i64;
    for (factor, weight) in &grand_factor {
        println!("grand factor {} * {}", factor, weight);
        grand_total *= factor * weight;
    }
    println!("grand total: {}", grand_total);
}
