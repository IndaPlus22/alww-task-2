use std::io::{self, BufRead};
fn main() {
    do_everything();
}

fn do_everything() {
    let input = io::stdin();
    let mut lines = input.lock().lines();

    while let Some(line) = lines.next() {
        let input = line.unwrap();

        if input.len() == 0 {
            break;
        }
        let parsed_input = string_to_vec(&input);
        if parsed_input[0] > parsed_input[1] {
            println!("{}", parsed_input[0] - parsed_input[1])
        } else {
            println!("{}", parsed_input[1] - parsed_input[0])
        }
    }
}
fn string_to_vec(a: &String) -> Vec<u128> {
    //https://stackoverflow.com/questions/26536871/how-can-i-convert-a-string-of-numbers-to-an-array-or-vector-of-integers-in-rust
    let numbers: Vec<u128> = a
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Could not parse input!"))
        .collect();
    numbers
}
