
fn is_palindrome(n: u64) -> bool {
    let original = n.to_string();
    let reversed: String = original.chars().rev().collect();
    original == reversed
}

fn main() {
    let number = 12321; 
    if is_palindrome(number) {
        println!("{} є паліндромом.", number);
    } else {
        println!("{} не є паліндромом.", number);
    }
}