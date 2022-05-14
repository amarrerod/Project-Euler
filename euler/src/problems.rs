use factor::factor_include;

pub fn multiples_of_3_or_5(limit: i32) -> i32 {
    let sum = (0..limit).filter(|x| x % 3 == 0 || x % 5 == 0).sum();
    sum
}

pub fn even_fibonacci_numbers() -> i32 {
    let mut seq: Vec<i32> = vec![1, 2];
    let mut prev_1 = 1;
    let mut prev_2 = 2;
    let mut last = 0;
    loop {
        last = prev_1 + prev_2;
        seq.push(last);
        prev_1 = prev_2;
        prev_2 = last;
        if last >= 4000000 {
            break;
        }
    }
    let sum = seq.iter().filter(|x| *x % 2 == 0).sum();
    sum
}

pub fn largest_prime_factor(n: &mut i64) -> Option<i64> {
    let mut terms = vec![];
    let mut d = 2;
    while (d * d) <= *n {
        while *n % d == 0 {
            terms.push(d);
            *n /= d;
        }
        d += 1;
    }
    if *n > 1 {
        terms.push(*n);
    }
    terms.sort();
    terms.pop()
}

/**
 * 2520 is the smallest number that can be divided
 * by each of the numbers from 1 to 10 without any remainder.
 * What is the smallest positive number that is evenly divisible
 * by all of the numbers from 1 to 20?
 */

fn _is_divisible(n: i32) -> bool {
    for i in 1..=20 {
        if n % i != 0 {
            return false;
        }
    }
    true
}

pub fn evenly_divisible() -> i32 {
    let mut smallest = i32::MAX;
    for i in 20..i32::MAX {
        if _is_divisible(i) && i < smallest {
            smallest = i;
            println!("Smallest is now: {}", smallest);
        }
    }
    smallest
}

/**
 * A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
 * a^2 + b^2 = c^2
 * For example, 3^2 + 4^2 = 9 + 16 = 25 = 5^2
 * There exists exactly one Pythagorean triplet for which a + b + c = 1000.
 * Find the product abc.
 */
pub fn pitagorean() {}

fn _is_prime(n: &i128) -> bool {
    for a in 2..*n {
        if n % a == 0 {
            return false; // if it is not the last statement you need to use `return`
        }
    }
    true // last value to return
}
/**
 * The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
 * Find the sum of all the primes below two million.
 */
pub fn sum_of_primes() -> i128 {
    let limit: i128 = 2000000;
    let sum = (2..=limit).filter(|x| _is_prime(&x)).sum();
    sum
}

pub fn highly_divisible_triangular() -> i64 {
    let mut factors: Vec<i64> = vec![];
    let mut number = 1;
    let mut iteration = 1;
    while factors.len() < 500 {
        number = (iteration * (iteration + 1)) / 2;
        factors = factor_include::factor_include(number);
        iteration += 1;
    }
    number
}
/**
*   The sum of the squares of the first ten natural numbers is,
*   The square of the sum of the first ten natural numbers is,
*   Hence the difference between the sum of the squares of the
*   first ten natural numbers and the square of the sum is.
*   Find the difference between the sum of the squares of
*   the first one hundred natural numbers and the square of the sum.
*/
pub fn sum_square_difference() -> i32 {
    let nums: Vec<i32> = (1..=100).collect();
    let sum_of_squares: i32 = nums.iter().map(|x| x * x).sum();
    let mut square_sum: i32 = nums.iter().sum();
    square_sum *= square_sum;
    square_sum - sum_of_squares
}

pub fn find_prime_in_position(limit: i32) -> i128 {
    let mut counter = 0;
    let mut last_prime = 2;
    let mut number = 2;
    while counter != limit {
        if _is_prime(&number) {
            last_prime = number;
            counter += 1;
            println!("Prime number {} is {}", counter, last_prime);
        }
        number += 1;
    }
    last_prime
}
