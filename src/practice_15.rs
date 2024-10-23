
fn gray(n: u8) -> Vec<String> {
    let mut result = Vec::new();
    
   
    for i in 0..(1 << n) { 
        let gray_code = i ^ (i >> 1); 
        result.push(format!("{:0width$b}", gray_code, width = n as usize)); 
    }

    result
}

fn main() {
    let n = 3; 
    let gray_codes = gray(n);

    println!("Код Грея для n = {}:", n);
    for code in gray_codes {
        println!("{}", code);
    }
}