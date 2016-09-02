/**
   The prime factors of 13195 are 5, 7, 13 and 29.
   What is the largest prime factor of the number 600851475143 ?
**/

fn factorize(value)
{
    let mut factors = [];
    for i in 2..value {
       if value % i == 0 {
           factors.push(i)
    }
    factors
}

fn main()
{
    let factors = factorize(600851475143);
    println!(max(factors))
}


