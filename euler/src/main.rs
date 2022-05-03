fn multiples_of_3_or_5(limit: i32) -> i32 {
    let sum = (0..limit).filter(|x| x % 3 == 0 || x % 5 == 0).sum();
    sum
}
fn main() {
    println!("Hello, world!");
    let result = multiples_of_3_or_5(1000);
    println!("{}", result);
}
