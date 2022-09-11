use std::collections::HashSet;
use std::{
    convert::TryInto,
    io::{self, BufRead},
};

fn main() {
    let (n, parsed_input) = parse_input(read_user_input());
    let unique_names: HashSet<_> = parsed_input.iter().cloned().collect();
    println!("{}", unique_number_of_guests(n, unique_names))
}
fn read_user_input() -> String {
    //https://www.becomebetterprogrammer.com/rust-read-user-input-stdin/
    let mut user_input = String::new();
    let input = io::stdin();
    let mut lines = input.lock().lines();

    while let Some(line) = lines.next() {
        let input = line.unwrap();

        if input.len() == 0 {
            break;
        }
        user_input.push_str(" ");
        user_input.push_str(&input)
    }
    user_input
}

fn parse_input(input: String) -> (u8, Vec<String>) {
    let split_input: Vec<&str> = input.trim().split_whitespace().collect();
    let (number, names) = split_input.split_at(1);
    let names = names.to_vec().into_iter().map(|x| x.to_owned()).collect();
    let number: u8 = number[0].parse().expect("Error while parsing input!");
    (number, names)
}

fn unique_number_of_guests(n: u8, unorganized_names: HashSet<String>) -> u8 {
    let number_of_unorganized_names: u8 = unorganized_names.len().try_into().unwrap();
    if is_even(number_of_unorganized_names.into()) {
        (n * 2) - number_of_unorganized_names
    } else {
        (n * 2) - number_of_unorganized_names + 1
    }
}
fn is_even(input: usize) -> bool {
    if input % 2 == 0 {
        true
    } else {
        false
    }
}
