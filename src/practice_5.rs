
const WIDTH: usize = 30;  
const HEIGHT: usize = 16; 

fn main() {
    let mut envelope = String::new();

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
  
            if i == 0  i == HEIGHT - 1  j == 0  j == WIDTH - 1  j == i * 2 || j == WIDTH - 1 - i * 2 {
                envelope.push('*');
            } else {
                envelope.push(' ');
            }
        }
        envelope.push('\n');
    }

    print!("{}", envelope);
}