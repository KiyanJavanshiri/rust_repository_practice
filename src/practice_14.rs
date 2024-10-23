
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Rectangle {
    a: Point,
    b: Point,
}


fn rectangle_area(rect: &Rectangle) -> i32 {
    let width = (rect.b.x - rect.a.x).abs();
    let height = (rect.b.y - rect.a.y).abs();
    width * height
}


fn overlap_area(rect1: &Rectangle, rect2: &Rectangle) -> i32 {
    let x_overlap = (rect1.a.x.max(rect2.a.x) - rect1.b.x.min(rect2.b.x)).max(0);
    let y_overlap = (rect1.a.y.min(rect2.a.y) - rect1.b.y.max(rect2.b.y)).max(0);
    x_overlap * y_overlap
}


fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut total_area = 0;
    let mut overlap_area_total = 0;

  
    for i in 0..xs.len() {
        total_area += rectangle_area(&xs[i]);
    }

   
    for i in 0..xs.len() {
        for j in i + 1..xs.len() {
            overlap_area_total += overlap_area(&xs[i], &xs[j]);
        }
    }

   
    total_area - overlap_area_total
}


fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}


fn main() {
    let rectangles = test_data();
    let occupied_area = area_occupied(&rectangles);
    println!("Фактична зайнята площа: {}", occupied_area);
}