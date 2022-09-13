use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let (n, parsed_input) = get_parsed_user_input();
    let mut unique_persons: HashSet<(String, String)> = HashSet::with_capacity(n);
    for x in 0..n {
        unique_persons.insert((
            parsed_input.get(x).unwrap().to_string(),
            parsed_input.get(x + n).unwrap().to_string(),
        ));
    }
    println!("{:?}", unique_persons.len());
}

fn get_parsed_user_input() -> (usize, Vec<String>) {
    let input = io::stdin();
    let mut lines = input.lock().lines().map(|var| var.ok().unwrap());
    let number = lines.next().unwrap().parse::<usize>().unwrap();
    let names = lines.collect::<Vec<String>>();
    (number, names)
}
