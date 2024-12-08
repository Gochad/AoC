use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};


fn read_input() -> Vec<Vec<char>> {
    let file_path = "8_input.txt";

    let file = File::open(file_path).expect("cannot open the file");
    let reader = io::BufReader::new(file);

    let mut result: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("cannot read line");
        result.push(line.chars().collect());
    }

    result
}


fn generate_pairs(vec: &Vec<(i64, i64)>) -> Vec<((i64, i64), (i64, i64))> {
    let mut pairs = Vec::new();

    for (i, &item1) in vec.iter().enumerate() {
        for &item2 in &vec[i + 1..] {
            pairs.push((item1, item2));
        }
    }

    pairs
}

fn return_points(mut points: ((i64, i64), (i64, i64))) -> ((i64, i64), (i64, i64)) {
    if points.0.1 > points.1.1 {
        points = (points.1, points.0);
    }
    if points.0.0 < points.1.0 {
        points = (points.0, points.1);
    }

    let dx = points.1.0 - points.0.0;
    let dy = points.1.1 - points.0.1;

    let x1 = points.0.0;
    let y1 = points.0.1;
    let x2 = points.1.0;
    let y2 = points.1.1;

    let y_left = (x1 - dx, y1 - dy);
    let y_right = (x2 + dx, y2 + dy);

    (y_left, y_right)
}

fn check_point(point: (i64, i64), rows: i64, columns: i64) -> bool {
    point.0 >= 0 && point.0 < columns && point.1 >= 0 && point.1 < rows
}

fn return_points_to_edge(
    mut points: ((i64, i64), (i64, i64)),
    rows: i64,
    columns: i64,
) -> Vec<(i64, i64)> {
    if points.0.1 > points.1.1 {
        points = (points.1, points.0);
    }
    if points.0.0 < points.1.0 {
        points = (points.0, points.1);
    }

    let dx = points.1.0 - points.0.0;
    let dy = points.1.1 - points.0.1;

    let mut result = Vec::new();

    let mut x = points.0.0;
    let mut y = points.0.1;

    while x >= 0 && x < columns && y >= 0 && y < rows {
        result.push((x, y));
        x -= dx;
        y -= dy;
    }

    x = points.1.0;
    y = points.1.1;

    while x >= 0 && x < columns && y >= 0 && y < rows {
        result.push((x, y));
        x += dx;
        y += dy;
    }

    result
}


fn main() {
    let matrix = read_input();

    let rows: i64 = matrix.len() as i64;
    let columns: i64 = matrix[0].len() as i64;

    let mut antennas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    let mut unique_points: HashSet<(i64, i64)> = HashSet::new();

    for (y, row) in matrix.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell != '.' {
                antennas
                    .entry(*cell)
                    .or_insert_with(Vec::new)
                    .push((x as i64, y as i64));
            }
        }
    }

    antennas.retain(|_, list| list.len() > 1);


    for (_, list) in &antennas {
        for pair in generate_pairs(list) {
            let points = return_points(pair);

            if check_point(points.0, rows, columns) {
                unique_points.insert(points.0);
            }

            if check_point(points.1, rows, columns) {
                unique_points.insert(points.1);
            }
        }
    }

    let sum = unique_points.len() as i64;

    println!("{}", sum);

    //2
    let mut unique_points: HashSet<(i64, i64)> = HashSet::new();

    for (_, list) in &antennas {
        for pair in generate_pairs(list) {
            let points = return_points_to_edge(pair, rows, columns);

            unique_points.extend(points); 
        }
    }

    let sum = unique_points.len() as i64;

    println!("{}", sum);
}