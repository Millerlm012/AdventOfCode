// use std::fs;
use std::fs::read_to_string;

fn main() {
    let rows: Vec<String> = read_to_string("./input.txt")
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect(); // gather them together into a vector

    let mut left = Vec::new();
    let mut right = Vec::new();
    for row in rows {
        let s = row.split("   ").collect::<Vec<&str>>();
        let left_num: i32 = s[0].parse().expect("Not a valid number");
        let right_num: i32 = s[1].parse().expect("Not a valid number");
        left.push(left_num);
        right.push(right_num);
    }

    left.sort();
    right.sort();

    let mut sum_diff = 0;
    for i in 0..left.len() {
        let diff = (left[i] - right[i]).abs();
        sum_diff += diff;
    }

    println!("{}", sum_diff);
}
