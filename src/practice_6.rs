
fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
fn main() {
    let num1 = 37;
    let num2 = 11;
    let result = gcd(num1, num2);
    println!("НСД чисел {} та {} дорівнює {}", num1, num2, result);
}