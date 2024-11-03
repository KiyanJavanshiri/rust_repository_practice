/// https://practice.course.rs/ownership/ownership.html
// ? 5.1 Ownership
// ! 1
fn main() {
    let x = String::from("Hello world");
    let y = x.clone(); 

    println!("{}, {}", x, y);
}

// ! 2
fn main() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}


fn take_ownership(s: String) -> String {
    println!("{}", s);
    s 
}

// ! 3
fn main() {
    let s = give_ownership();
    println!("{}", s);
}


fn give_ownership() -> String {
    let s = String::from("Hello world");
    
    
    let _s = s.as_bytes().to_vec(); 

    s 
}

// ! 4
fn main() {
    let s = String::from("Hello World");

    print_str(&s); 

    println!("{}", s); 
}

fn print_str(s: &String) { 
    println!("{}", s);
}

// ! 5
fn main() {
    let x = (1, 2, (), "hello"); 
    let y = x; 
    println!("{:?}, {:?}", x, y);
}

// * Mutability
// ! 6
fn main() {
    let s = String::from("Hello ");
    
    let mut s1 = s; 

    s1.push_str("World!"); 

    println!("{}", s1); 
}

// ! 7
fn main() {
    let mut x = Box::new(5); 
    
    let y = &mut x; 
    
    **y = 4; 

    assert_eq!(*x, 4); 

    println!("Success!");
}

// * Partial move
// ! 8
fn main() {
    let t = (String::from("hello"), String::from("world"));

    let _s = t.0;

   
    println!("{:?}", (&t.1)); 
}

// ! 9
fn main() {
    let t = (String::from("hello"), String::from("world"));

    
    let (s1, s2) = (t.0.clone(), t.1.clone()); 

    println!("{:?}, {:?}, {:?}", s1, s2, t); 
}

// ? 5.2 Reference and Borrowing
// * Reference
// ! 1
fn main() {
    let x = 5;
    
    let p = &x; 

    println!("the memory address of x is {:p}", p); 
}
// ! 2
fn main() {
    let x = 5;
    let y = &x;

  
    assert_eq!(5, *y); 

    println!("Success!");
}

// ! 3
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s); 

    println!("Success!");
}

fn borrow_object(s: &String) {} 

// ! 4
fn main() {
    let mut s = String::from("hello, ");

    push_str(&mut s); 

    println!("Success! {}", s); 
}

fn push_str(s: &mut String) {
    s.push_str("world"); 
}

// ! 5
fn main() {
    let mut s = String::from("hello, ");

   
    let p = &mut s;
    
    p.push_str("world"); 

    println!("Success! {}", s); 
}

// * Ref
// ! 6
fn main() {
    let c = '中';

    let r1 = &c;
   
    let r2 = &c; 
    
    assert_eq!(*r1, *r2);
    

    assert_eq!(get_addr(r1), get_addr(r2));

    println!("Success!");
}


fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}

// * Borrowing rules
// ! 7
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
  
  

    println!("{}", r1); 

    println!("Success!");
}

// * Mutability
// ! 8
fn main() {
 
    let mut s = String::from("hello, "); 

    borrow_object(&mut s); 

    println!("Success!");
}

fn borrow_object(s: &mut String) {} 

// ! 9
fn main() {
    let mut s = String::from("hello, ");

    borrow_object(&s);
    
    s.push_str("world");

    println!("Success!");
}

fn borrow_object(s: &String) {} 

// * NLL
// ! 10
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");


    println!("{}", r1); /
}

// ! 11
fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s; 
    let r2 = &mut s; 


    r1.push_str("world"); 
    r2.push_str("!");     
}
