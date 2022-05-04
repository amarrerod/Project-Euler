mod problems;
use problems::*;
fn main() {
    println!("Hello, world!");
    let result = problems::multiples_of_3_or_5(1000);
    let result = problems::even_fibonacci_numbers();
    println!("{}", result);
}
