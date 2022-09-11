use std::io;
fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error!");
    makes_rectangle(string_to_vector(input));
}
fn string_to_vector(input: String) -> Vec<u16> {
    let numbers_vecor = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Error while parsing!"))
        .collect();
    numbers_vecor
}

fn makes_rectangle(input: Vec<u16>) {
    let length_x = input[0];
    let length_y = input[1];
}
