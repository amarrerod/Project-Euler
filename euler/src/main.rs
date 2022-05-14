mod adjacents;
mod palindrome;
mod problems;

use problems::*;

use factor::factor_include;

fn main() {
    println!("Hello, world!");
    /*  let mut result = problems::multiples_of_3_or_5(1000);
    result = problems::even_fibonacci_numbers();
    let mut number: i64 = 600851475143;
    let prime_result = problems::largest_prime_factor(&mut number).expect("Not found");
    //let result_palindrome = palindrome::largest_palindrome_product();
    let smallest = problems::evenly_divisible(); */
    //let adjancent_sum = adjacents::solve_8_problem("src/data.txt");
    //println!("sum is {}", adjancent_sum);
    //let sum_of_primes = problems::sum_of_primes();
    //println!("{}", sum_of_primes);
    let result = problems::highly_divisible_triangular();
    println!("{}", result);
}
