// Copyright © 2017 Bart Massey

// Sort an array of random numbers with quicksort
// and print the median.

extern crate quicksort;
extern crate rand;
use rand::Rng;

fn main() {
    let usage = "usage: median <count>";
    let n = std::env::args().nth(1).expect(usage);
    let n = n.parse().expect(usage);
    if n <= 0 {
        panic!(usage)
    }
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(rand::thread_rng().gen_range(1, 2 * n))
    }
    quicksort::quicksort(&mut a);
    println!("{}", a[n / 2])
}
