
use std::iter;

fn draw_triangle(base_width: usize, max_width: usize) {
    let height = (base_width + 1) / 2;

    for i in 0..height {
        let spaces = iter::repeat(' ').take(max_width - i - 1).collect::<String>();
        let stars = iter::repeat('*').take(2 * i + 1).collect::<String>();
        println!("{}{}", spaces, stars);
    }
}

fn draw_tree(triangles: usize) {
    let mut max_width = 0;
    
   
    for i in 1..=triangles {
        max_width += 2;  
    }
    
    let mut current_base_width = 1;
    for _ in 1..=triangles {
        draw_triangle(current_base_width, max_width);
        current_base_width += 2; 
    }
}

fn main() {
    let triangles = 5; 
    draw_tree(triangles);
}

