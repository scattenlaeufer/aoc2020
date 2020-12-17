use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>{
    let mut file = File::open("input")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let string_vec = content.split("\n").collect::<Vec<&str>>();
    let number_iter = string_vec.iter().map(|s| s.to_string().parse::<u32>().unwrap_or_default());
    let total_number_vec = number_iter.collect::<Vec<u32>>();
    let number_vec = &total_number_vec[..total_number_vec.len()-1];
    for i in 0..number_vec.len()-1 {
        for j in &number_vec[i+1..] {
            if number_vec[i] + j == 2020 {
                println!("Solution 1: {} * {} = {}", number_vec[i], j, number_vec[i]*j);
            }
        }
    }
    for i in 0..number_vec.len()-2 {
        for j in i..number_vec.len()-1 {
            for k in &number_vec[j+1..] {
                if number_vec[i] + number_vec[j] + k == 2020 {
                    println!("Solution 2: {} * {} * {} = {}", number_vec[i], number_vec[j], k, number_vec[i]*number_vec[j]*k);
                }
            }
        }
    }

    Ok(())
}
