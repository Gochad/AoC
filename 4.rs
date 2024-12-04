use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

fn read_input() -> Vec<Vec<char>> {
    let file_path = "4_input.txt";

    let file = File::open(file_path).expect("cannot open the file");
    let reader = io::BufReader::new(file);

    let mut result: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("cannot read line");
        result.push(line.chars().collect());
    }

    result
}

fn check_word(word: &[char]) -> bool {
    let word_str: String = word.iter().collect();
    word_str == "XMAS" || word_str.chars().rev().collect::<String>() == "XMAS"
}

fn check_matrix(word: &[Vec<char>]) -> bool {
    let ms_re = Regex::new(r"M.S").unwrap();
    let a_re = Regex::new(r".A.").unwrap();

    word.iter()
        .map(|row| row.iter().collect::<String>())
        .zip(&[&ms_re, &a_re, &ms_re])
        .all(|(row, re)| re.is_match(&row))
}

fn rotate_matrix_90(matrix: &[Vec<char>]) -> Vec<Vec<char>> {
    let n = matrix.len();
    let mut rotated = vec![vec![' '; n]; n];
    for i in 0..n {
        for j in 0..n {
            rotated[j][n - 1 - i] = matrix[i][j];
        }
    }
    rotated
}

fn main() {
    let matrix = read_input();

    let mut sum: i64 = 0;

    let rows = matrix.len();
    let cols = matrix[0].len();

    for i in 0..rows {
        for j in 0..=cols - 4 {
            if check_word(&matrix[i][j..j + 4]) {
                sum += 1;
            }
        }
    }

    for i in 0..=rows - 4 {
        for j in 0..cols {
            let word: Vec<char> = (0..4).map(|k| matrix[i + k][j]).collect();
            if check_word(&word) {
                sum += 1;
            }
        }
    }

    for i in 0..=rows - 4 {
        for j in 0..=cols - 4 {
            let word: Vec<char> = (0..4).map(|k| matrix[i + k][j + k]).collect();
            if check_word(&word) {
                sum += 1;
            }
        }
    }

    for i in 3..rows {
        for j in 0..=cols - 4 {
            let word: Vec<char> = (0..4).map(|k| matrix[i - k][j + k]).collect();
            if check_word(&word) {
                sum += 1;
            }
        }
    }

    println!("{}", sum);


    // 2
    sum = 0;
    for i in 0..rows - 2 {
        for j in 0..=cols - 3 {
            let word: Vec<Vec<char>> = matrix[i..i + 3]
                .iter()
                .map(|row| row[j..j + 3].to_vec())
                .collect();

            if check_matrix(&word) {
                sum += 1;
            }

            let mut rotated = word.clone();
            for _ in 0..3 {
                rotated = rotate_matrix_90(&rotated);
                if check_matrix(&rotated) {
                    sum += 1;
                }
            }
        }
    }

    println!("{}", sum);
}
