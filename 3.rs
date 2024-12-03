use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;
use std::io::Read;

fn read_input() -> String {
    let file_path = "3_input.txt";

    let mut file = File::open(file_path).expect("cannot open the file");
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("cannot read the file");

    contents
}

fn main() {
    let text = "do()".to_owned() + &read_input();
    let mult_re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let number_re = Regex::new(r"\d+").unwrap();

    let mut sum: i64 = 0;

    for mult_find in mult_re.find_iter(&text) {
        let numbers = number_re.find_iter(mult_find.as_str());

        let mut x: i64 = 1;

        for i in numbers {
            let num: i64 = i.as_str().parse().expect("cannot parse number");
            x *= num;
        }
        
        sum += x;
    }
    

    println!("{:?}", sum);

    //2
    let range_re = Regex::new(r"(do\(\)|don't\(\)|mul\((\d+),\s*(\d+)\))").unwrap();
    sum = 0;

    let mut is_enabled = true;
    let mut sum = 0;

    for cap in range_re.captures_iter(&text) {
        let instruction = cap.get(1).unwrap().as_str();

        if instruction == "do()" {
            is_enabled = true;
        } else if instruction == "don't()" {
            is_enabled = false;
        } else if is_enabled {
            let x: i64 = cap.get(2).unwrap().as_str().parse().unwrap();
            let y: i64 = cap.get(3).unwrap().as_str().parse().unwrap();
            sum += x * y;
        }
    }
    
    println!("{:?}", sum);  
}