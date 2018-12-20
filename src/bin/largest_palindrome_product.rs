/**
 * Problem 4: Largest palindrome product
 * A palindromic number reads the same both ways. The largest palindrome made from the
 * product of two 2-digit numbers is 9009 = 91 × 99.
 * Find the largest palindrome made from the product of two 3-digit numbers.
**/
extern crate euler;

fn main() {
    let mut largest = 1_i32;
    for x in 1..100 {
        for y in 1..100 {
            let product = x * y;
            if product == euler::reverse_digits(product) && product > largest {
                largest = product;
                println!("x: {}, y: {}, largest: {}", x, y, largest)
            }
        }
    }
}
