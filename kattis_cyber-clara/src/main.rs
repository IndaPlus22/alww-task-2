use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let (n, parsed_input) = parse_input(read_user_input());
    // let (n, parsed_input) = correct_way();
    let tmp: usize = n.into();
    let mut persons = Vec::<(String, String)>::new();
    for x in 0..n {
        let x_to_usize: usize = x.into();
        persons.push((
            parsed_input.get(x_to_usize).unwrap().to_string(),
            parsed_input.get(x_to_usize + tmp).unwrap().to_string(),
        ));
    }
    let unique_persons: HashSet<_> = persons.iter().clone().collect();
    println!("{:?}", unique_persons.len());
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
    let number: u8 = number[0].parse().expect("Error while parsing input!");
    let names = names
        .to_vec()
        .into_iter()
        .map(|x| x.to_owned())
        .collect::<Vec<String>>();
    (number, names)
}

fn correct_way() -> (u8, Vec<String>) {
    let input = io::stdin();
    let mut lines = input.lock().lines().map(|var| var.ok().unwrap());
    let number = lines.next().unwrap().parse::<u8>().unwrap();
    let names = lines.collect::<Vec<String>>();
    (number, names)
}
