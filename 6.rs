use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

fn read_input() -> Vec<Vec<char>> {
    let file_path = "6_input.txt";

    let file = File::open(file_path).expect("cannot open the file");
    let reader = io::BufReader::new(file);

    let mut result: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("cannot read line");
        result.push(line.chars().collect());
    }

    result
}

fn move_to_obstacle(matrix: &Vec<Vec<char>>, position_x: &mut usize, position_y: &mut usize, rotation: &mut usize, moved: &mut bool) {
    for _ in 0..4 {
        if *rotation == 0 {
            if *position_x > 0 && matrix[*position_x - 1][*position_y] != '#' {
                *position_x -= 1;
                *moved = true;
                break;
            } else if *position_x == 0 {
                *moved = false;
                break;
            } else {
                *rotation = (*rotation + 90) % 360;
            }
        } else if *rotation == 90 {
            if *position_y + 1 < matrix[*position_x].len() && matrix[*position_x][*position_y + 1] != '#' {
                *position_y += 1;
                *moved = true;
                break;
            } else if *position_y + 1 == matrix[*position_x].len() {
                *moved = false;
                break;
            } else {
                *rotation = (*rotation + 90) % 360;
            }
        } else if *rotation == 180 {
            if *position_x + 1 < matrix.len() && matrix[*position_x + 1][*position_y] != '#' {
                *position_x += 1;
                *moved = true;
                break;
            } else if *position_x + 1 == matrix.len() {
                *moved = false;
                break;
            } else {
                *rotation = (*rotation + 90) % 360;
            }
        } else if *rotation == 270 {
            if *position_y > 0 && matrix[*position_x][*position_y - 1] != '#' {
                *position_y -= 1;
                *moved = true;
                break;
            } else if *position_y == 0 {
                *moved = false;
                break;
            } else {
                *rotation = (*rotation + 90) % 360;
            }
        }
    }
}

fn creates_loop(matrix: &Vec<Vec<char>>, start_x: usize, start_y: usize) -> bool {
    let mut visited_states = HashSet::new();
    let mut position_x = start_x;
    let mut position_y = start_y;
    let mut rotation = 0;

    loop {
        if visited_states.contains(&(position_x, position_y, rotation)) {
            return true;
        }
        visited_states.insert((position_x, position_y, rotation));

        let mut moved = false;

        // for i in 0..matrix.len() {
        //     println!("{}", matrix[i].iter().collect::<String>());
        // }
        // println!("{:?} {:?} {:?}", position_x, position_y, rotation);


        if rotation == 0 {
            if position_x > 0 && matrix[position_x - 1][position_y] != '#' {
                position_x -= 1;
                moved = true;
            } else if position_x == 0 {
                return false;
            } else {
                rotation = (rotation + 90) % 360;
            }
        } else if rotation == 90 {
            if position_y + 1 < matrix[position_x].len() && matrix[position_x][position_y + 1] != '#' {
                position_y += 1;
                moved = true;
            } else if position_y + 1 == matrix[position_x].len() {
                return false;
            } else {
                rotation = (rotation + 90) % 360;
            }
        } else if rotation == 180 {
            if position_x + 1 < matrix.len() && matrix[position_x + 1][position_y] != '#' {
                position_x += 1;
                moved = true;
            } else if position_x + 1 == matrix.len() {
                return false;
            } else {
                rotation = (rotation + 90) % 360;
            }
        } else if rotation == 270 {
            if position_y > 0 && matrix[position_x][position_y - 1] != '#' {
                position_y -= 1;
                moved = true;
            } else if position_y == 0 {
                return false;
            } else {
                rotation = (rotation + 90) % 360;
            }
        }

    }
}

fn main() {
    let mut matrix = read_input();
    let matrix_cloned = matrix.clone();

    let mut position_x = 0;
    let mut position_y = 0;
    let mut rotation = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '^' {
                position_x = i;
                position_y = j;
                break;
            }
        }
    }
    let mut start_x = position_x;
    let mut start_y = position_y;

    let mut sum: i64 = 1;
    matrix[position_x][position_y] = 'X';

    while position_x < matrix.len() && position_y < matrix[position_x].len() {
        let mut moved = false;
    
        move_to_obstacle(&matrix, &mut position_x, &mut position_y, &mut rotation, &mut moved);
    
        if moved {
            if matrix[position_x][position_y] != 'X' {
                matrix[position_x][position_y] = 'X';
                sum += 1;
            }
        } else {
            break;
        }
    }

    println!("{:?}", sum);

    // 2
    sum = 0;

    for i in 0..matrix_cloned.len() {
        for j in 0..matrix_cloned[i].len() {
            if matrix_cloned[i][j] == '.' && !(i == start_x && j == start_y) {
                let mut test_matrix = matrix_cloned.clone();
                test_matrix[i][j] = '#';

                if creates_loop(&test_matrix, start_x, start_y) {
                    sum += 1;
                }
            }
        }
    }
    println!("{:?}", sum); 
}