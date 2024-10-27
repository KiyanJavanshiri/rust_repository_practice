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