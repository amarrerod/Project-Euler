/*A palindromic number reads the same both ways.
The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
Find the largest palindrome made from the product of two 3-digit numbers.
*/
fn is_palindrome(i: i32) -> bool {
    let i_str = i.to_string();
    let reverse = i_str.chars().rev().collect::<String>();
    i_str == reverse
}

pub fn largest_palindrome_product() -> i32 {
    let mut largest = 0;
    for i in 100..=999 {
        for j in 100..=999 {
            let product = i * j;
            if is_palindrome(product) && product > largest {
                largest = product;
            }
        }
    }
    largest
}
