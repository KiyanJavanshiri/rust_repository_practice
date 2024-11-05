// ? Pattern Match
// * Match, if let
// ! 1
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North  => { 
            println!("South or North");
        },
        _ => println!("West"),
    };
}

// ! 2
fn main() {
    let boolean = true;

    let binary = match boolean {
        true => 1,
        false => 0
    };

    assert_eq!(binary, 1);
}

// ! 3
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msgs = [
        Message::Quit,
        Message::Move{x:1, y:3},
        Message::ChangeColor(255,255,0)
    ];

    for msg in msgs {
        show_message(msg)
    }
} 

fn show_message(msg: Message) {
    match msg {
        Message::Move{x: a, y: b} => { 
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        },
        Message::ChangeColor(_, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        }
        _ => println!("no data in these variants")
    }
}

// ! 4
fn main() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }
}

// ! 5
enum MyEnum {
    Foo,
    Bar
}

fn main() {
    let mut count = 0;

    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
    for e in v {
        if matches!(e , MyEnum::Foo) { 
            count += 1;
        }
    }

    assert_eq!(count, 2);
}

// ! 6
fn main() {
    let o = Some(7);

    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
    }
}

// ! 7
enum Foo {
    Bar(u8)
}

fn main() {
    let a = Foo::Bar(1);

    if let Foo::Bar(i) = a {
        println!("foobar holds the value: {}", i);
    }
}

// ! 8
enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn main() {
    let a = Foo::Qux(10);

    match a {
        Foo::Bar => println!("match foo::bar"),
        Foo::Baz => println!("match foo::baz"),
        _ =>  println!("match others")
    }
}

// ! 9
fn main() {
    let age = Some(30);
    if let Some(age) = age {
       assert_eq!(age, 30);
    }
    
    match age {
    
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }
 }

//  * Patters
// ! 1
fn main() {}
fn match_number(n: i32) {
    match n {
       
        1 => println!("One!"),
       
        2 | 3 | 4 | 5 => println!("match 2 -> 5"),
        
        6..=10 => {
            println!("match 6 -> 10")
        },
        _ => {
            println!("match 11 -> +infinite")
        }
    }
}

// ! 2
struct Point {
    x: i32,
    y: i32,
}

fn main() {

    let p = Point { x: 2, y: 20 }; 

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
  
        Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

// ! 3
enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id:  id@3..=7,
        } => println!("Found an id in range [3, 7]: {}", id),
        Message::Hello { id: newid@(10 | 11 | 12) } => {
            println!("Found an id in another range [10, 12]: {}", newid)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

// ! 4
fn main() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) if x < split => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }
}

// ! 5
fn main() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        (first,..,last) => {
           assert_eq!(first, 2);
           assert_eq!(last, 2048);
        }
    }
}

// ! 6
fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
        value => value.push_str(" world!"), 
    }

    println!("{}", v); 
}
