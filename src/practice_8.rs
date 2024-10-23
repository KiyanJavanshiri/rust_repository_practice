
fn swap_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}

fn main() {
    let text = "Hello, World!";
    let swapped_text = swap_case(text);
    println!("Original: {}", text);
    println!("Swapped: {}", swapped_text);
}