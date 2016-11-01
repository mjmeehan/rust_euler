/**
 *  2520 is the smallest number that can be divided by each of the numbers
 *  from 1 to 10 without any remainder.
 *  What is the smallest positive number that is evenly divisible by all of
 *  the numbers from 1 to 20?
**/
extern crate euler;
use std::cmp;
use std::collections::btree_map::BTreeMap;

fn factor(number: i32) -> BTreeMap<i32, i32>
{

}

fn main()
{
    let grand_factor = BTreeMap.new();
    for divisor in 1..20 {
        factors = factor(divisor);
        for (factor, weight) in &factors {
           grand_factor[factor] = cmp::max(weight, grand_factor[factor])
        }
    }
}
