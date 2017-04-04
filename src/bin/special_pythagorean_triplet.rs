/**
 *  Problem 9: Special Pythagorean triplet
 *  A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
 *  a^2 + b^2 = c^2
 *  For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
 *  There exists exactly one Pythagorean triplet for which a + b + c = 1000.
 *  Find the product abc.
 **/
extern crate euler;
extern crate rand;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};

fn is_pythagorean(Triplet trip) -> bool
{
    a * a + b * b == c * c
}

struct Triplet {
  a: i32,
  b: i32,
  c: i32
}

fn yield_triplet() -> Triplet
{
    let between = Range::new(1, 998);
 

}

fn main()
{
  for 
    
}

