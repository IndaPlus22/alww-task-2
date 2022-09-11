use std::fmt;
use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error!");
    io::stdin().read_line(&mut input).expect("Error!");
    let numbers = string_to_vec(input);
    println!("{}", compare_sum_2_greatest_values(numbers));
}
fn compare_sum_2_greatest_values(mut a: Vec<u32>) -> u32 {
    a.sort_by(|a, b| b.cmp(a));
    let a_length = a.len();
    if is_even(a_length) {
        a.truncate(a_length / 2);
    } else {
        a.truncate((a_length + 1) / 2);
    };
    a.iter().sum()
}

fn string_to_vec(a: String) -> Vec<u32> {
    //https://stackoverflow.com/questions/26536871/how-can-i-convert-a-string-of-numbers-to-an-array-or-vector-of-integers-in-rust
    let mut numbers: Vec<u32> = a
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Could not parse input!"))
        .collect();
    numbers.remove(0);
    numbers
}
fn is_even(input: usize) -> bool {
    if input % 2 == 0 {
        true
    } else {
        false
    }
}
