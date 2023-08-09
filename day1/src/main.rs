use std::fs::read_to_string;
fn main() {
    println!("Hello, world!");
    let lines = read_lines("./input.txt");
    let mut calories: Vec<i32> = Vec::new();
    let mut temp_calories = 0;
    let mut curr: i32;
    for line in lines.iter() {
        if line.is_empty() {
            calories.push(temp_calories);
            temp_calories = 0;
        } else {
            curr = line.parse().unwrap();
            temp_calories += curr;
        }
    }
    calories.sort();
    println!("{}", calories[calories.len() - 1] + calories[calories.len() - 2] + calories[calories.len() - 3]);
    println!("{}", calories[calories.len() - 1]);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
