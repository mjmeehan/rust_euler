/**
   The prime factors of 13195 are 5, 7, 13 and 29.
   What is the largest prime factor of the number 600851475143 ?
**/
use std::f64;

fn factorize(factorant:i64) -> Vec<i64>
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

fn main()
{
    let factors = factorize(600851475143);
    for factor in &factors {
        println!("factor: {}", factor);
    }

    let largest_factor = factors.iter().max();
    match largest_factor {
        Some(factor) => println!("factors: {}", factor),
        None => println!("Can't happen, no factors"),
    }
}


