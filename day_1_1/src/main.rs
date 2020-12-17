use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()>{
    let mut file = File::open("input")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let string_vec = content.split("\n").collect::<Vec<&str>>();
    let number_iter = string_vec.iter().map(|s| s.to_string().parse::<u32>().unwrap_or_default());
    let number_vec = number_iter.collect::<Vec<u32>>();
    for i in 0..number_vec.len()-1 {
        for j in &number_vec[i+1..] {
            if number_vec[i] + j == 2020 {
                println!("{} * {} = {}", number_vec[i], j, number_vec[i]*j);
            }
        }
    }

    Ok(())
}
