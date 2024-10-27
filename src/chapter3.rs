/// https://practice.course.rs/variables.html
// * Binding and mutability
// ! 1
fn main() {
    let x: i32 = 5; 
    let y: i32; 

    assert_eq!(x, 5);
    println!("Success!");
}

// ! 2
fn main() {
    let mut x = 1; 
    x += 2; 
    
    assert_eq!(x, 3);
    println!("Success!");
}

// * Scope
// ! 3
fn main() {
    let x: i32 = 10;
    let y: i32; 

    {
        y = 5; 
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y); 
}

// ! 4
fn main() {
    let x = define_x(); 
    println!("{}, world", x);
}

fn define_x() -> &'static str { 
    let x = "hello";
    x 
}

// * Shadowing
// ! 5
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12); 
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); 
}

// ! 6
fn main() {
    let mut x: i32 = 1;
    x = 7;
    x += 3;
    let y = 4;    
    let y = "I can also be bound to text!"; 

    println!("Success!");
}

// * Unused variables
// ! 7
fn main() {
    let x = 1; 
    println!("{}", x); 
}

// * Destructuring
// ! 8
fn main() {
    let (mut x, y) = (1, 2); 
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}

// * Destructuring assignments
// ! 9
fn main() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    
    assert_eq!([x, y], [3, 2]); 

    println!("Success!");
}