use std::fs::File;
use std::io::{self, BufRead};

fn read_input() -> Vec<Vec<i64>> {
    let file_path = "2_input.txt";

    let file = File::open(file_path).expect("cannot open the file");
    let reader = io::BufReader::new(file);

    let mut numbers: Vec<Vec<i64>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("cannot read line");
        let parts: Vec<&str> = line.split_whitespace().collect();

        let mut row: Vec<i64> = Vec::new();

        for part in &parts {
            let num: i64 = part.parse().expect("cannot parse number");
            row.push(num);
        }

        numbers.push(row);
    }

    numbers
}

fn is_safe(report: &Vec<i64>) -> bool {
    let is_decreasing = report[0] > report[1];
    
    for i in 0..report.len() - 1 {
        let diff = (report[i + 1] - report[i]).abs();

        if diff < 1 || diff > 3 {
            return false;
        }

        if is_decreasing && report[i + 1] >= report[i] {
            return false;
        }
        if !is_decreasing && report[i + 1] <= report[i] {
            return false;
        }
    }

    true
}

fn is_safe2(report: &Vec<i64>) -> bool {
    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);
        if is_safe(&modified_report) {
            return true;
        }
    }

    false
}

fn main() {
    let matrix = read_input();

    let mut count: i64 = 0;

    for report in &matrix {
        if is_safe(report) {
            count += 1;
        }
    }
    // 1
    println!("{:?}", count);

    count = 0;

    for report in &matrix {
        if is_safe2(report) {
            count += 1;
        }
    }
    // 2
    println!("{:?}", count);
}

