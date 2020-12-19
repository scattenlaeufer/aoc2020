use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct GroupAnswers {
    answered_yes: Vec<char>,
}

impl GroupAnswers {
    fn new(answers: &str) -> Self {
        let mut answered_yes = Vec::new();
        for member in answers.split("\n") {
            for answer in member.chars() {
                if !answered_yes.contains(&answer) {
                    answered_yes.push(answer)
                }
            }
        }
        GroupAnswers { answered_yes }
    }
}
fn main() {
    let mut file = File::open("input").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let string_vec = content.split("\n\n").collect::<Vec<&str>>();
    let mut group_vec = Vec::new();
    let mut total_yes_answers = 0;
    for group in string_vec {
        let group = GroupAnswers::new(group);
        total_yes_answers += group.answered_yes.len();
        group_vec.push(group);
    }
    println!("Total yes answers: {}", total_yes_answers);
}
