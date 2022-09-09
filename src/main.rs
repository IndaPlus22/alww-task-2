extern crate is_even;
use is_even::IsEven;
use std::fmt;
use std::io;

fn main() {
    let mut input: String = String::new();
    fmt::write(&mut input, format_args!("5 1 1 1"));
    // io::stdin().read_line(&mut input).expect("Error!");
    let numbers = string_to_vec(input);
    println!("{}", compare_sum_2_greatest_values(numbers));
}
fn compare_sum_2_greatest_values(mut a: Vec<u32>) -> u32 {
    a.sort_by(|a, b| b.cmp(a));
    let a_length = a.len();
    if a_length.is_even() {
        a_length + 1
    } else {
        a_length
    };
    a.truncate();
    assert_eq!(a, [5, 1]);
    a.iter().sum()
}

fn string_to_vec(a: String) -> Vec<u32> {
    //https://stackoverflow.com/questions/26536871/how-can-i-convert-a-string-of-numbers-to-an-array-or-vector-of-integers-in-rust
    let numbers = a
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Could not parse input!"))
        .collect();
    numbers
}
