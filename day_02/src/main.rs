use std::fs::File;
use std::io::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut file = File::open("input").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let string_vec = content.split("\n").collect::<Vec<&str>>();

    let mut part_1_valid_passwords = 0;
    let mut part_2_valid_passwords = 0;

    for rule_string in &string_vec[..string_vec.len() - 1] {
        let input_split = rule_string.split(": ").collect::<Vec<&str>>();
        let password = input_split[1];
        let rule = input_split[0].split(" ").collect::<Vec<&str>>();
        let letter = rule[1];
        let letter_amount = rule[0].split("-").collect::<Vec<&str>>();
        let low = letter_amount[0].to_string().parse::<usize>().unwrap();
        let high = letter_amount[1].to_string().parse::<usize>().unwrap();
        let amount = password.matches(letter).count();
        if low <= amount && amount <= high {
            part_1_valid_passwords += 1;
        }

        let mut segmented_password = UnicodeSegmentation::graphemes(password, true);
        println!("{:?} {} {}-{}", password, letter, low, high);
        println!(
            "{:?} - {:?}",
            segmented_password.clone().nth(low - 1),
            segmented_password.clone().nth(high - 1)
        );
        if (segmented_password.clone().nth(low - 1).unwrap() == letter)
            ^ (segmented_password.nth(high - 1).unwrap() == letter)
        {
            part_2_valid_passwords += 1;
            println!("true");
        } else {
            println!("false");
        }
    }

    println!("part 1 valid passwords: {}", part_1_valid_passwords);
    println!("part 2 valid passwords: {}", part_2_valid_passwords);
}
