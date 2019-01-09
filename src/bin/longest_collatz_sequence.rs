/**
Longest Collatz sequence
Problem 14
The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:

13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?

NOTE: Once the chain starts the terms are allowed to go above one million.

**/
use std::collections::btree_map::BTreeMap;

type CollatzNum = u64;
type CollatzResult = (CollatzNum, usize);
type CollatzCache = BTreeMap<CollatzNum, CollatzResult>;


fn collatz(x: CollatzNum) -> CollatzNum {
    if x == 1 {
        1
    } else if x % 2 == 0 {
        // even
        x / 2
    } else {
        // odd
        3 * x + 1
    }
}

fn collatz_chain(start: CollatzNum, cache: &mut CollatzCache) -> CollatzResult
{
    if let Some(&(end, depth)) = cache.get(&start) { // in cache?
        return (end, depth);
    }
    let mut prev = start.clone();
    let mut chain = Vec::new();
    loop {
        //print!(" {}", prev);
        // base case
        if prev == 1 {
            let result = (1, 0);
            println!("...base case, caching ({}, {:?})", prev, result);
            cache.insert(prev, result);
        } else {
            chain.push(prev);
        }
        // In cache
        if let Some(&(end, depth)) = cache.get(&prev) {
            let step = chain.len();
            //println!("...in cache, {} more", depth);
            // unwind chain, add to cache incrementing depth
            cache_chain(cache, &mut chain, depth, end);
            //println!();
            return (end, depth + step);
        }
        let next = collatz(prev);
        //print!(" -> ");
        prev = next;
    }
}

fn cache_chain(cache: &mut CollatzCache, chain: &mut Vec<CollatzNum>, depth: usize, prev_link: CollatzNum) {
    //print!("Caching chaendin: ");
    let mut prev_link = prev_link;
    for (ordinal, &link) in chain.iter().rev().enumerate(){
        let result = (prev_link, depth + ordinal);
        //print!("({}, {:?}), ", link, result);
        cache.insert(link, result);
        prev_link = link;
    }
}

fn main() {
    let mut cache = CollatzCache::new();
    //for start in 1..1_000_000 {
    let mut longest = (1, 0);
    for start in 1..1_000_001 {
        let (next, depth) = collatz_chain(start, &mut cache);
        if depth > longest.1 {
            println!("New longest chain: start={}, next={}, depth={}", start, next, depth);
            longest = (next, depth);
        }

    }
}
