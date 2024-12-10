use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashSet, HashMap, VecDeque};

fn read_input(file_path: &str) -> Vec<Vec<i64>> {
    let file = File::open(file_path).expect("cannot open the file");
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| {
            line.expect("cannot read line")
                .chars()
                .filter_map(|ch| ch.to_digit(10))
                .map(|digit| digit as i64)
                .collect()
        })
        .collect()
}

fn bfs_trailhead(map: &[Vec<i64>], start_x: usize, start_y: usize) -> usize {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut routes = HashSet::new();

    queue.push_back((start_x, start_y, 0));
    visited.insert((start_x, start_y));

    while let Some((x, y, height)) = queue.pop_front() {
        if map[x][y] == 9 {
            routes.insert((x, y));
            continue;
        }

        for (dx, dy) in directions.iter() {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;

            if nx < map.len() && ny < map[0].len() && !visited.contains(&(nx, ny)) {
                let next_height = map[nx][ny];
                if next_height == height + 1 {
                    queue.push_back((nx, ny, next_height));
                    visited.insert((nx, ny));
                }
            }
        }
    }

    routes.len()
}

fn calculate_total_score(map: &[Vec<i64>]) -> usize {
    map.iter().enumerate().fold(0, |sum, (x, row)| {
        sum + row.iter().enumerate().filter(|&(_, &height)| height == 0).map(|(y, _)| bfs_trailhead(map, x, y)).sum::<usize>()
    })
}

fn dfs(x: usize, y: usize, map: &[Vec<i64>], cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if map[x][y] == 9 {
        return 1;
    }

    if let Some(&cached_result) = cache.get(&(x, y)) {
        return cached_result;
    }

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let total_trails = directions.iter().filter_map(|&(dx, dy)| {
        let nx = (x as isize + dx) as usize;
        let ny = (y as isize + dy) as usize;

        if nx < map.len() && ny < map[0].len() && map[nx][ny] == map[x][y] + 1 {
            Some(dfs(nx, ny, map, cache))
        } else {
            None
        }
    }).sum();

    cache.insert((x, y), total_trails);
    total_trails
}

fn calculate_total_rating(map: &[Vec<i64>]) -> usize {
    map.iter().enumerate().fold(0, |total_rating, (x, row)| {
        total_rating + row.iter().enumerate().filter(|&(_, &height)| height == 0).map(|(y, _)| {
            let mut cache = HashMap::new();
            dfs(x, y, map, &mut cache)
        }).sum::<usize>()
    })
}

fn main() {
    let input = read_input("10_input.txt");
    let sum = calculate_total_score(&input);
    println!("{}", sum);

    let rating = calculate_total_rating(&input);
    println!("{}", rating);
}
