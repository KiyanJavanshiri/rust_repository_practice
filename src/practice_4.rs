
fn main() {
    const SIZE: usize = 6; 

  
    for i in 0..SIZE {
       
        for _ in 0..SIZE - i - 1 {
            print!(" ");
        }
 
        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!(); 
    }


    for i in (0..SIZE - 1).rev() {

        for _ in 0..SIZE - i - 1 {
            print!(" ");
        }

        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!(); 
    }
}
// commit test