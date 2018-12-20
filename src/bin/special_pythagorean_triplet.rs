/**
 *  Problem 9: Special Pythagorean triplet
 *  A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
 *  a^2 + b^2 = c^2
 *  For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2.
 *  There exists exactly one Pythagorean triplet for which a + b + c = 1000.
 *  Find the product abc.
 **/
extern crate euler;

fn is_pythagorean(a: i32, b: i32, c: i32) -> bool {
    a * a + b * b == c * c
}

fn main() {
    for not_c in 1..998 {
        let c = 1000 - not_c;
        for a in 1..not_c {
            let b = not_c - a;
            if is_pythagorean(a, b, c) {
                println!("{} + {} = {}; product {}", a, b, c, a * b * c);
            }
        }
    }
}
