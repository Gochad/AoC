use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

fn read_input() -> io::Result<HashMap<i64, Vec<i64>>> {
    let file_path = "7_input.txt";

    let file = File::open(file_path).expect("cannot open the file");
    let reader = io::BufReader::new(file);

    let mut result: HashMap<i64, Vec<i64>> = HashMap::new();

    for line in reader.lines() {
        let line = line.expect("cannot read line");
        let sections: Vec<&str> = line.split(":").collect();

        let key = sections[0].trim().parse().expect("cannot parse key");
        let values: Vec<i64> = sections[1]
            .split_whitespace()
            .map(|v| v.trim().parse().expect("cannot parse value"))
            .collect();

        result.entry(key).or_insert_with(Vec::new).extend(values);
    }

    Ok(result)
}

fn all_possible_results(nums: &[i64]) -> Vec<i64> {
    if nums.is_empty() {
        return vec![];
    }

    let mut current = vec![nums[0]];

    for &num in &nums[1..] {
        let mut next = Vec::new();
        for &val in &current {
            next.push(val + num);
            next.push(val * num);
        }
        current = next;
    }

    current
}

fn evaluate_expression(nums: &[i64], target: i64) -> bool {
    let results = all_possible_results(nums);
    results.contains(&target)
}

//2
fn all_possible_results2(nums: &[i64]) -> Vec<i64> {
    if nums.is_empty() {
        return vec![];
    }

    let mut current = vec![nums[0]];

    for &num in &nums[1..] {
        let mut next = Vec::new();
        for &val in &current {
            next.push(val + num);
            next.push(val * num);
            next.push(format!("{}{}", val, num).parse().unwrap());
        }
        current = next;
    }

    current
}

fn evaluate_expression2(nums: &[i64], target: i64) -> bool {
    let results = all_possible_results2(nums);
    results.contains(&target)
}

fn main() {
    let values = match read_input() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error reading input: {}", e);
            return;
        }
    };

    let mut result: i64 = 0;

    for (key, list) in &values {
        if evaluate_expression(list, *key) {
            result += key;
        }
    }

    println!("{}", result);

    //
    result = 0;

    for (key, list) in &values {
        if evaluate_expression2(list, *key) {
            result += key;
        }
    }

    println!("{}", result);
}
