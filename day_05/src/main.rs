use std::fs::File;
use std::io::prelude::*;

fn find_seat(seat_vec: Vec<u8>, path: &[char], low: char, high: char) -> u8 {
    if seat_vec.len() == 1 {
        seat_vec[0]
    } else {
        if path[0] == low {
            find_seat(
                seat_vec[..seat_vec.len() / 2].to_vec(),
                &path[1..],
                low,
                high,
            )
        } else {
            find_seat(
                seat_vec[seat_vec.len() / 2..].to_vec(),
                &path[1..],
                low,
                high,
            )
        }
    }
}

fn get_seat_id(boarding_pass: &str) -> u16 {
    let row_path = boarding_pass[..7].chars().collect::<Vec<char>>();
    let row = find_seat((0..128).collect(), &row_path, 'F', 'B');
    let column_path = boarding_pass[7..].chars().collect::<Vec<char>>();
    let column = find_seat((0..8).collect(), &column_path, 'L', 'R');
    row as u16 * 8 + column as u16
}

fn main() {
    let mut file = File::open("input").unwrap();
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    let string_vec = content.split("\n").collect::<Vec<&str>>();

    let mut highest_seat_id = 0;
    let mut seat_id_vec = Vec::new();

    for boarding_pass in &string_vec[..string_vec.len() - 1] {
        let seat_id = get_seat_id(boarding_pass);
        seat_id_vec.push(seat_id);
        if seat_id > highest_seat_id {
            highest_seat_id = seat_id;
        }
    }

    println!("Highest seat id: {}", highest_seat_id);
    seat_id_vec.sort();

    let mut missing_seat_ids = Vec::new();

    for i in 0..128 * 8 {
        if !(seat_id_vec.contains(&i)) {
            missing_seat_ids.push(i);
        }
    }

    println!("missing seat ids: {:?}", missing_seat_ids);
}
