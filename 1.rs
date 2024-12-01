use std::fs::File;
use std::io::{self, BufRead};

fn read_input() -> (Vec<i64>, Vec<i64>) {
    let file_path = "1_input.txt";

    let file = File::open(file_path).expect("cannot open the file");
    let reader = io::BufReader::new(file);

    let mut first_numbers: Vec<i64> = Vec::new();
    let mut second_numbers: Vec<i64> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("cannot read line");
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            let num1: i64 = parts[0].parse().expect("cannot parse first number");
            let num2: i64 = parts[1].parse().expect("cannot parse second number");
            first_numbers.push(num1);
            second_numbers.push(num2);
        }
    }

    (first_numbers, second_numbers)
}

fn main() {
    let (mut first_list, mut second_list) = read_input();
    first_list.sort();
    second_list.sort();

    let mut sum: i64 = 0;
    
    for i in 0..first_list.len() {
        sum += (first_list[i] - second_list[i]).abs();
    }

    println!("{:?}", sum); 
}