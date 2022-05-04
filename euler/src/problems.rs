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
