// ? newtype and DST
// ! 1
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

// ! 2
struct Meters(u32);

impl Meters {
    fn value(&self) -> u32 {
        self.0
    }

    fn pow(&self, exp: u32) -> u32 {
        self.value().pow(exp) 
    }
}

fn main() {
    let i: u32 = 2;
    assert_eq!(i.pow(2), 4);

    let n = Meters(i);
    assert_eq!(n.pow(2), 4);
}

// ! 3
struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}


fn old_enough(age: &Years) -> bool {
    age.0 >= 18
}
fn old_enough_in_days(age: &Days) -> bool {
    let years = age.to_years();
    old_enough(&years)
}

fn main() {
    let age = Years(5);
    let age_days = age.to_days();
    
    println!("Old enough (in years): {}", old_enough(&age));
    println!("Old enough (in days): {}", old_enough_in_days(&age_days)); 
}

// ! 4
use std::ops::Add;
use std::fmt;

struct Meters(u32);

impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There are still {} meters left", self.0)
    }
}

impl Add for Meters {
    type Output = Self;

    fn add(self, other: Meters) -> Self {
        Self(self.0 + other.0)
    }
}

fn calculate_distance(m1: Meters, m2: Meters) -> Meters {
    m1 + m2 
}

fn main() {
    let d = calculate_distance(Meters(10), Meters(20));
    assert_eq!(format!("{}", d), "There are still 30 meters left");

    println!("Success!"); 
}

// ! 5
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    let x = Operations::Add;
}

// ! 6
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            VeryVerboseEnumOfThingsToDoWithNumbers::Add => x + y,
            VeryVerboseEnumOfThingsToDoWithNumbers::Subtract => x - y,
        }
    }
}

fn main() {
    let add_operation = VeryVerboseEnumOfThingsToDoWithNumbers::Add;
    let subtract_operation = VeryVerboseEnumOfThingsToDoWithNumbers::Subtract;

    println!("Add: {}", add_operation.run(5, 3));         
    println!("Subtract: {}", subtract_operation.run(5, 3)); 
}

// ! 7
fn my_function<const N: usize>() -> [u32; N] {
    [123; N]
}

fn main() {
    let arr = my_function::<5>(); 
    println!("{:?}", arr);
}

// ! 8
fn main() {
    let s: &str = "Hello there!"; 

    let arr: [u8; 3] = [1, 2, 3]; 

    println!("{}", s); 
    println!("{:?}", arr); 
}

// ! 9
use std::fmt::Display;

fn foobar<T: Display>(thing: T) {
    println!("{}", thing); 
}

fn main() {
    foobar("Hello, world!");
    foobar(42); 
}
// Тест

