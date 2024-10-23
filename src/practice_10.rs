
fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;
    

    if len == 0 {
        return s;
    }

  
    let shift = ((n % len) + len) % len;

 
    let split_point = shift as usize;
    let rotated = [&s[split_point..], &s[..split_point]].concat();

    rotated
}

fn main() {
    let s = String::from("abcdef");
    let rotated = rotate(s.clone(), 2); 
    println!("Rotated right: {}", rotated);

    let rotated_left = rotate(s.clone(), -2); 
    println!("Rotated left: {}", rotated_left);
}
