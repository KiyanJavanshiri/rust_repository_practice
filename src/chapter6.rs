/// https://practice.course.rs/compound-types/string.html
// ? 6.1 String
// * Str and &str
// ! 1
fn main() {
    let s: &str = "hello, world"; 

    println!("Success!");
}

// ! 2
fn main() {
    let s: Box<str> = "hello, world".into();
    greetings(&s); 
}

fn greetings(s: &str) {
    println!("{}", s)
}

// * String
// ! 3
fn main() {
    let mut s = String::new(); 
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

// ! 4
fn main() {
    let mut s = String::from("hello"); 
    s.push(',');                       
    s.push_str(" world");              
    s += "!";                           

    println!("{}", s);
}

// ! 5
fn main() {
    let s = String::from("I like dogs");
  
    let s1 = s.replace("dogs", "cats"); 

    assert_eq!(s1, "I like cats");

    println!("Success!");
}

// ! 6
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2; 
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}

// * &str and String
// ! 7
fn main() {
    let s = "hello, world";
    greetings(s.to_string()); 
}

fn greetings(s: String) {
    println!("{}", s);
}

// ! 8
fn main() {
    let s = "hello, world".to_string();
    let s1: &str = &s; 

    println!("Success!");
}

// * String escapes
// ! 9
fn main() {

    let byte_escape = "I'm writing Ru\x73\x74!";
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
                unicode_codepoint, character_name );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

// ! 10
fn main() {
    let raw_str = r"Escapes don't work here: ? ℝ"; 
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

 

    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    
    let long_delimiter = r#"Hello, "##""#; 
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");
}

// * String index
// ! 11
fn main() {
    let s1 = String::from("hi,中国");
    
    
    let h = s1.chars().nth(0).unwrap(); 
    assert_eq!(h.to_string(), "h"); 


    let h1 = &s1[3..4]; 
    assert_eq!(h1, "中");

    println!("Success!");
}

// * Operate on UTF8 string
// ! 12
fn main() {
   
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
}

// ? 6.2 Array
// ! 1
fn main() {

    let arr: [i32; 5] = [1, 2, 3, 4, 5]; 

   
    assert!(arr.len() == 5); 

    println!("Success!");
}

// ! 2
fn main() {
 
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];
    
   
    assert!(std::mem::size_of_val(&arr) == 12); 

    println!("Success!");
}

// ! 3
fn main() {
  
    let mut list: [i32; 100] = [0; 100]; 
    list[0] = 1; 

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success!");
}

// ! 4
fn main() {
 
    let _arr = ['1', '2', '3']; 

    println!("Success!");
}

// ! 5
fn main() {
    let arr = ['a', 'b', 'c'];
    
    let ele = arr[0]; 

    assert!(ele == 'a');

    println!("Success!");
}

// ! 6
fn main() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    
  
    let name0 = names.get(0).unwrap();

   
    let _name1 = &names[1]; 

    println!("Success!");
}

// ?  6.3 Slice
// ! 1
fn main() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world"; 

    println!("Success!");
}

// ! 2
fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

  
    assert!(std::mem::size_of_val(&slice) == 16); 

    println!("Success!");
}

// ! 3
fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    let slice: &[i32] = &arr[1..4]; 
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success!");
}

// * String slices
// ! 4
fn main() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    
    let slice2 = &s[..2]; 

    assert_eq!(slice1, slice2);

    println!("Success!");
}

// ! 5
fn main() {
    let s = "你好，世界";
 
    let slice = &s[0..3]; 

    assert!(slice == "你");

    println!("Success!");
}

// ! 6
fn main() {
    let mut s = String::from("hello world");

    let letter = first_letter(&s); 

    s.clear(); 

    println!("the first letter is: {}", letter);
}

fn first_letter(s: &str) -> String {
    s[..1].to_string() 
}

// ? 6.4 Tuple
// ! 1
fn main() {
    let _t0: (u8, i16) = (0, -1);

    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    
   
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success!");
}

// ! 2
fn main() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface"); 

    println!("Success!");
}

// ! 3
fn main() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11); 
    println!("too long tuple: {:?}", too_long_tuple);
}

// ! 4
fn main() {
    let tup = (1, 6.4, "hello");


    let (x, z, y) = tup; 

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success!");
}

// ! 5
fn main() {
    let (x, y, z);

  
    (y, z, x) = (1, 2, 3); 
    
    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Success!");
}

// ! 6
fn main() {

    let (x, y) = sum_multiply((2, 3)); 

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success!");
}

fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

// ? 6.5 Struct
// * The types of structs
// ! 1
struct Person {
    name: String,
    age: u8,
    hobby: String,
}

fn main() {
    let age = 30;
    let p = Person {
        name: String::from("sunface"),
        age,
        hobby: String::from("coding"), 
    };

    println!("Success!");
}

// ! 2
struct Unit;
trait SomeTrait {
  
}

impl SomeTrait for Unit { }

fn main() {
    let u = Unit;
    do_something_with_unit(u);

    println!("Success!");
} 


fn do_something_with_unit(u: Unit) { }

// ! 3
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let v = Point(0, 127, 255); 
    check_color(Color(v.0, v.1, v.2)); 

    println!("Success!");
}

fn check_color(p: Color) {
    let Color(x, y, z) = p; 
    assert_eq!(x, 0);
    assert_eq!(y, 127);
    assert_eq!(z, 255); 
}

// * Operating on structs
// ! 4
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let age = 18;
    let mut p = Person { 
        name: String::from("sunface"),
        age,
    };

    
    p.age = 30;

  
    p.name = String::from("sunfei"); 

    println!("Success!");
}

// ! 5
struct Person {
    name: String,
    age: u8,
}

fn main() {
    println!("Success!");
} 

fn build_person(name: String, age: u8) -> Person {
    Person {
        age,
        name, 
    }
}

// ! 6
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let u1 = User {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email(u1);

    println!("Success!");
} 

fn set_email(u: User) -> User {
    User {
        email: String::from("contact@im.dev"),
        username: u.username, 
        active: u.active,
        sign_in_count: u.sign_in_count,
    }
}

// * Print the structs
// ! 7
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1); 

    println!("{:#?}", rect1); 
}

// * Partial move
// ! 8
