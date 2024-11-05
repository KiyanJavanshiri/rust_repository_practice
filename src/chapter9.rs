// ? Associated functions & Methods
// * Method
// ! 1
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
 
    fn area(&self) -> u32 { 
        self.width * self.height 
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    assert_eq!(rect1.area(), 1500); 

    println!("Success!");
}

// ! 2
#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(&self)  {
        println!("the current state is {}", self.color);
    }
}
fn main() {
    let light = TrafficLight{
        color: "red".to_owned(),
    };
  
    light.show_state();

    println!("{:?}", light);
}

// ! 3
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    
    pub fn show_state(self: &Self) { 
        println!("the current state is {}", self.color); 
    }

   
    pub fn change_state(&mut self) { 
        self.color = "green".to_string(); 
    }
}

fn main() {
    let mut light = TrafficLight {
        color: "red".to_string(),
    };

    light.show_state(); 
    light.change_state(); 
    light.show_state(); 

    println!("Success!");
}

// * Associated functions
// ! 4
#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn new() -> Self { 
        Self { color: "red".to_string() } 
    }

    pub fn get_state(&self) -> &str {
        &self.color
    }
}

fn main() {
    let light = TrafficLight::new();
    assert_eq!(light.get_state(), "red");

    println!("Success!");
}

// * Multiple impl blocks
// ! 5
struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}


impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    println!("Success!");
}

// * Enums
// ! 6
#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

impl TrafficLightColor {
    pub fn color(&self) -> &str { 
        match self {
            TrafficLightColor::Red => "red",
            TrafficLightColor::Yellow => "yellow",
            TrafficLightColor::Green => "green",
        }
    }
}

fn main() {
    let c = TrafficLightColor::Yellow;

    assert_eq!(c.color(), "yellow");

    println!("{:?}", c);
}

