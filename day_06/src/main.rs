use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct GroupAnswers {
    answered_yes: Vec<char>,
    answered_yes_by_everyone: Vec<char>,
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
        let members = answers.split("\n").collect::<Vec<&str>>();
        let mut answered_yes_by_everyone = members[0].chars().collect::<Vec<char>>();
        for member in &members[1..] {
            let answer_chars = member.chars().collect::<Vec<char>>();
            for answer in answered_yes_by_everyone.clone() {
                if !answer_chars.contains(&answer) {
                    if let Some(i) = answered_yes_by_everyone.iter().position(|x| x == &answer) {
                        answered_yes_by_everyone.remove(i);
                    }
                };
            }
        }
        GroupAnswers {
            answered_yes,
            answered_yes_by_everyone,
        }
    }
}
fn main() {
    let mut file = File::open("input").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let string_vec = content[..content.len() - 1]
        .split("\n\n")
        .collect::<Vec<&str>>();
    let mut group_vec = Vec::new();
    let mut total_yes_answers = 0;
    let mut total_yes_answers_by_everyone = 0;
    for group in string_vec {
        println!("{}", group);
        let group = GroupAnswers::new(group);
        total_yes_answers += group.answered_yes.len();
        total_yes_answers_by_everyone += group.answered_yes_by_everyone.len();
        println!("{:#?}", group);
        group_vec.push(group);
    }
    println!("Total yes answers: {}", total_yes_answers);
    println!(
        "Total yes answers by everyone in a group: {}",
        total_yes_answers_by_everyone
    );
}
