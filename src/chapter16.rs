// ? Formatted output
// * 16.1 println! and format!
// ! 1
fn main() {
    let s1 = "hello";
    let s = format!("{}, world!", s1);
    assert_eq!(s, "hello, world!");
}

// ! 2
fn main() {
    print!("hello world, ");
    println!("I am");
    println!("Sunface!");
}

// * 16.2 Debug and Display
// ! 1
#[derive(Debug)]
struct Structure(i32);

fn main() {
    println!("{:?} months in a year.", 12);

    println!("Now {:?} will print!", Structure(3));
}

// ! 2
#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

fn main() {
    let person = Person { name:  "Sunface".to_string(), age: 18 };

    println!("{:#?}", person);
}

// ! 3
use std::fmt;

struct Structure(i32);

struct Deep(Structure);
impl fmt::Debug for Deep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0.0)
    }
}

fn main() {    
    println!("Now {:?} will print!", Deep(Structure(7)));
}

// ! 4
use std::fmt;

struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display: {} + {}i", self.x, self.y)
    }
}

impl fmt::Debug for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Debug: Complex {{ real: {:?}, imag: {:?} }}", self.x, self.y)
    }
}

fn main() {
    let point = Point2D { x: 3.3, y: 7.2 };

    assert_eq!(format!("{}",point), "Display: 3.3 + 7.2i");
    assert_eq!(format!("{:?}",point), "Debug: Complex { real: 3.3, imag: 7.2 }");
    
    println!("Success!")
}

// ! 5
use std::fmt; 

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let vec = &self.0;

        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}",count, v)?;
        }

        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    assert_eq!(format!("{}",v), "[0: 1, 1: 2, 2: 3]");
}

// * Formatting
// ! 1
fn main() {
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    assert_eq!(format!("{1}{0}", 1, 2), "21");
    assert_eq!(format!("{1}{}{0}{}", 1, 2), "2112");
    println!("Success!")
}

// ! 2
fn main() {
    println!("{argument}", argument = "test"); 

    assert_eq!(format!("{name}{}", 1, name = 2), "21");
    assert_eq!(format!("{a} {c} {b}",a = "a", b = 'b', c = 3 ), "a 3 b");
    
    println!("{abc} {0}", 2, abc = "def");

    println!("Success!")
}

// ! 3
fn main() {
    println!("Hello {:5}!", "x"); 
    println!("Hello {:1$}!", "x", 5); 

    assert_eq!(format!("Hello {1:0$}!", 5, "x"), "Hello x    !");
    assert_eq!(format!("Hello {:width$}!", "x", width = 5), "Hello x    !");

    println!("Success!")
}

// ! 4
fn main() {
    println!("Hello {:<5}!", "x");
    assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
    assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");

    assert_eq!(format!("Hello {:&<5}!", "x"), "Hello x&&&&!");

    println!("Success!")
}

// ! 5
fn main() {
    println!("Hello {:5}!", 5); 
    println!("Hello {:+}!", 5);
    println!("Hello {:05}!", 5); 
    println!("Hello {:05}!", -5);

    assert!(format!("{number:0>width$}", number=1, width=6) == "000001");
    
    println!("Success!")
}

// ! 6
fn main() {
    let v = 3.1415926;

    println!("{:.1$}", v, 4); 

    assert_eq!(format!("{:.2}", v), "3.14");
    assert_eq!(format!("{:+.2}", v), "+3.14");
    assert_eq!(format!("{:.0}", v), "3");

    println!("Success!")
}

// ! 7
fn main() {
    let s = "Hello, world!";

    println!("{0:.5}", s); 

    assert_eq!(format!("Hello {1:.0$}!", 3, "abcdefg"), "Hello abc!");

    println!("Success!")
}

// ! 8
fn main() {
    assert_eq!(format!("{:#b}", 27), "0b11011");
    assert_eq!(format!("{:#o}", 27), "0o33");
    assert_eq!(format!("{:#x}", 27), "0x1b");
    assert_eq!(format!("{:#X}", 27), "0x1B");

    println!("{:x}!", 27); 

    println!("{:#010b}", 27); 

   println!("Success!")
}

// ! 9
fn get_person() -> String {
    String::from("sunface")
}

fn get_format() -> (usize, usize) {
    (4, 1)
}


fn main() {
    let person = get_person();
    println!("Hello, {person}!");

    let (width, precision) = get_format();
    let scores = [("sunface", 99.12), ("jack", 60.34)];

    for (name, score) in scores {
        println!("{name}: {score:width$.precision$}");
    }
}