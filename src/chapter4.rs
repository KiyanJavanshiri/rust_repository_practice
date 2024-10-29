/// https://practice.course.rs/basic-types/numbers.html
// ? 4.1 Numbers
// * Integer
// ! 1
fn main() {
    let x: i32 = 5;
    let mut y: u32 = 5;    
    let z = 10; 

    println!("Success!");
}

// ! 2
fn main() {
    let v: u16 = 38_u8 as u16; 

    println!("Success!");
}

// ! 3
fn main() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x)); 

    println!("Success!");
}


fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

// ! 4
fn main() {
    assert_eq!(i8::MAX, 127); 
    assert_eq!(u8::MAX, 255); 

    println!("Success!");
}

// ! 5
fn main() {
    let v1 = 251_u16 + 8;
    let v2 = u8::checked_add(251, 8).unwrap();
    println!("{},{}",v1,v2);
 }

//  ! 6
fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597); 
    println!("Success!");
}

// * Floating-Point
// ! 7
fn main() {
    let x: f64 = 1_000.000_1; 
    let y: f32 = 0.12; 
    let z = 0.01_f64; 

    assert_eq!(type_of(&x), "f64".to_string());
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

// ! 8
fn main() {
    assert!(1.0 + 2.0 == 3.0);

    println!("Success!");
}


// * Range
// ! 9
fn main() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i;
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}", c as u32);
    }
}

// ! 10
use std::ops::{Range, RangeInclusive};
fn main() {
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

// * Computations
// ! 11
fn main() {
    
    assert!(1u32 + 2 == 3);


    assert!(1i32 - 2 == -1);
    
    
    assert!(3 * 50 == 150);

    
    assert!((9.6_f32 / 3.2 - 3.0).abs() < f32::EPSILON);

    
    assert!(24 % 5 == 4);

    
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101); 
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101); 
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101); 
    println!("1 << 5 is {}", 1u32 << 5);                    
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);          
}

// ? 4.2 Char, Bool and Unit
// * Char
// ! 1
use std::mem::size_of_val;

fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4); 

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4); 

    println!("Success!");
}

// ! 2
fn main() {
    let c1 = "中";
    let c1_char = c1.chars().next().unwrap(); 
    print_char(c1_char);
} 

fn print_char(c: char) {
    println!("{}", c);
}

// * Bool
// ! 3

