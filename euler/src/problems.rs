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
*   The sum of the squares of the first ten natural numbers is,
*   The square of the sum of the first ten natural numbers is,
*   Hence the difference between the sum of the squares of the 
*   first ten natural numbers and the square of the sum is.
*   Find the difference between the sum of the squares of 
*   the first one hundred natural numbers and the square of the sum.
*/
pub fn sum_square_difference() -> i32 {
    let nums : Vec<i32> = (1..=100).collect();
    let sum_of_squares : i32 = nums.iter().map(|x| x*x).sum();
    let mut square_sum : i32 = nums.iter().sum();
    square_sum *= square_sum;
    square_sum - sum_of_squares
}

fn _is_prime(n: i32) -> bool {
    for i in (2..n-1).rev() {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn find_prime_in_position(limit: i32) -> i32 {
    let mut counter = 0;
    let mut last_prime = 2;
    let mut number = 2;
    while counter != limit {
        if _is_prime(number) {
            last_prime = number;
            counter += 1;
            println!("Prime number {} is {}", counter, last_prime);
        }
        number += 1;
    }
    last_prime
}