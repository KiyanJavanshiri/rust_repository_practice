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
fn main() {
    let _f: bool = false;

    let t = true;
    if t {
        println!("Success!");
    }
}

// ! 4
fn main() {
    let f = true;
    let t = true || false; 
    assert_eq!(t, f);

    println!("Success!");
}

// * Unit type
// ! 5
fn main() {
    let _v: () = ();

    let v = (); 
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// ! 6
use std::mem::size_of_val;

fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0); 

    println!("Success!");
}

// ? 4.3 Statements and Expressions
// ! 1
fn main() {
    let v = {
        let mut x = 1;
        x += 2;
        x 
    };
 
    assert_eq!(v, 3);
 
    println!("Success!");
 }

// ! 2
fn main() {
    let v = {
        let x = 3;
        x
    };
 
    assert!(v == 3);
 
    println!("Success!");
 }

// ! 3
fn main() {
    let s = sum(1, 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y 
}

// ? 4.4 Functions
// ! 1
fn main() {

    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y 
}

// ! 2
fn main() {
    print();
 }
 

 fn print() -> () {
    println!("Success!");
 }
 
// ! 3
fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    panic!("This function never returns!");
}

// * Diverging functions
// ! 4
fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            
            Some(42)
        }
        _ => {
            
            never_return_fn();
        }
    }
}

fn never_return_fn() -> ! {
    
    loop {
        
    }
}

// ! 5
fn main() {
  
    let b = false; 

    let _v = match b {
        true => 1,
     
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");
}


