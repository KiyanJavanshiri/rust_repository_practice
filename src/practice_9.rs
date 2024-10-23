
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    
    true
}

fn main() {
    let number = 30; 
    if is_prime(number) {
        println!("{} є простим числом.", number);
    } else {
        println!("{} не є простим числом.", number);
    }
}

