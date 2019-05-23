// iterators4.rs


// Complete this function to return factorial of num
// Do not use:
// - return
// For extra fun don't use:
// - imperative style loops (for, while)
// - additional variables
// For the most fun don't use:
// - recursion
// Scroll down for hints.

pub fn factorial(num: u64) -> u64 {
    match num {
      n if n <= 1 => 1,
      n => n * factorial(n - 1)
    }
}

/*
here is an (outdated) non-recursive solution using iterators
https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/nightly/std/iter/trait.MultiplicativeIterator.html

use std::iter::{count, MultiplicativeIterator};

fn factorial(n: usize) -> usize {
    count(1, 1).take_while(|&i| i <= n).product()
}

TODO: find out where count and MultiplicativeIterator currently reside in the 
std library.
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}

























// In an imperative language you might write a for loop to iterate through
// multiply the values into a mutable variable. Or you might write code more
// functionally with recursion and a match clause. But you can also use ranges
// and iterators to solve this in rust.
