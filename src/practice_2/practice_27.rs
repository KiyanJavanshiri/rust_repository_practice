use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i32]) {
    let total_sum: i64 = arr.iter().map(|&x| x as i64).sum(); 
    let min_value = *arr.iter().min().unwrap(); 
    let max_value = *arr.iter().max().unwrap(); 

    let min_sum = total_sum - max_value as i64; 
    let max_sum = total_sum - min_value as i64; 

    println!("{} {}", min_sum, max_sum); 
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}