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
