/**
   The prime factors of 13195 are 5, 7, 13 and 29.
   What is the largest prime factor of the number 600851475143 ?
**/
extern crate euler;

fn main()
{
    let factors = euler::factorize(600851475143);
    for (factor, weight) in &factors {
        println!("factor: {}", *factor);
    }

    let largest_factor = factors.keys().max();
    match largest_factor {
        Some(factor) => println!("factors: {}", factor),
        None => println!("Can't happen, no factors"),
    }
}


