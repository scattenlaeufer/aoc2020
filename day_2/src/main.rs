use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("input").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let string_vec = content.split("\n").collect::<Vec<&str>>();

    let mut valid_passwords = 0;

    for rule_string in &string_vec[..string_vec.len()-1] {
        let input_split = rule_string.split(": ").collect::<Vec<&str>>();
        let password = input_split[1];
        let rule = input_split[0].split(" ").collect::<Vec<&str>>();
        let letter = rule[1];
        let letter_amount = rule[0].split("-").collect::<Vec<&str>>();
        let low = letter_amount[0].to_string().parse::<usize>().unwrap();
        let high = letter_amount[1].to_string().parse::<usize>().unwrap();
        let amount = password.matches(letter).count();
        if low <= amount && amount <= high {
            valid_passwords += 1;
        }
    }

    println!("valid passwords: {}", valid_passwords);
}
