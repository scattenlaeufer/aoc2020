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

    for line in map.iter() {
        if line[pos % line.len()] == tree {
            trees_hit += 1;
        }
        pos += 3;
    }

    println!("trees hit: {}", trees_hit);

    let slopes = [[1, 1], [1, 3], [1, 5], [1, 7], [2, 1]];
    let mut pos_vec = vec![0, 0, 0, 0, 0];
    let mut trees_hit_vec = vec![0, 0, 0, 0, 0];

    for i in 0..map.len() {
        for j in 0..slopes.len() {
            if i % slopes[j][0] == 0 {
                if map[i][pos_vec[j] % map[i].len()] == tree {
                    trees_hit_vec[j] += 1;
                }
                pos_vec[j] += slopes[j][1];
            }
        }
    }

    let mut trees_hit: i64 = 1;
    for h in trees_hit_vec {
        trees_hit = trees_hit * h;
    }
    println!("trees hit on all slopes multiplied: {}", trees_hit);
}
