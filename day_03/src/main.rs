use std::fs::File;
use std::io::prelude::*;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let mut file = File::open("input").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let string_vec = content.split("\n").collect::<Vec<&str>>();
    let mut map: Vec<Vec<&str>> = Vec::new();

    for line in &string_vec[..string_vec.len() - 1] {
        map.push(UnicodeSegmentation::graphemes(*line, true).collect());
    }

    let mut trees_hit = 0;
    let mut pos = 0;
    let tree = "#";

    for line in map {
        if line[pos % line.len()] == tree {
            trees_hit += 1;
        }
        pos += 3;
    }

    println!("trees hit: {}", trees_hit);
}
