use std::fs::File as OtherFile;
use std::io::{self, BufReader, Read};

#[derive(Debug, Clone)]
struct FreeSegment {
    start: usize,
    size: usize,
}

#[derive(Debug, Clone)]
struct File {
    id: i64,
    start: usize,
    size: usize,
}

fn read_input() -> String {
    let file_path = "9_input.txt";

    let file = OtherFile::open(file_path).expect("cannot open the file");
    let mut reader = BufReader::new(file);

    let mut content = String::new();

    reader.read_to_string(&mut content).expect("cannot read the file");

    content
}

fn calculate_checksum(disk: &[i64]) -> i64 {
    disk.iter()
        .enumerate()
        .filter(|&(_, &block)| block >= 0)
        .map(|(pos, &block)| pos as i64 * block)
        .sum()
}




fn main() {
    let matrix = read_input();

    let mut digits: Vec<i64> = Vec::new();

    for ch in matrix.chars() {
        if let Some(digit) = ch.to_digit(10) {
            digits.push(digit as i64);
        }
    }
    
    let total_size: i64 = digits.iter().sum();
    let mut unwrapped: Vec<i64> = vec![0; total_size as usize];

    let mut index: usize = 0;
    let mut position: usize = 0;

    for i in 0..digits.len() {
        if i % 2 == 0 {
            for j in position..(position + digits[i] as usize) {
                unwrapped[j] = index as i64;
            }
            index += 1;
        }
        position += digits[i] as usize;
    }

    let mut empty_pos = digits[0] as usize;
    let mut last_nonzero_index = unwrapped.iter().rposition(|&value| value != 0).unwrap();

    loop {
        while empty_pos < unwrapped.len() && unwrapped[empty_pos] > 0 {
            empty_pos += 1;
        }

        if empty_pos >= last_nonzero_index {
            break;
        }

        unwrapped[empty_pos] = unwrapped[last_nonzero_index];
        unwrapped[last_nonzero_index] = 0;

        if let Some(new_index) = unwrapped[..last_nonzero_index].iter().rposition(|&value| value != 0) {
            last_nonzero_index = new_index;
        } else {
            break;
        }

        empty_pos += 1;
    }

    let sum = calculate_checksum(&unwrapped);
    println!("{}", sum);

    //2
    let mut disk = parse_disk_map(&matrix);
    compact_disk(&mut disk);

    let sum = calculate_checksum(&disk);
    println!("{}", sum);
}

fn parse_disk_map(input: &str) -> Vec<i64> {
    let mut disk = Vec::new();
    let mut id_counter = 0;
    let chars: Vec<char> = input.chars().collect();

    for i in (0..chars.len()).step_by(2) {
        let file_size = chars[i].to_digit(10).unwrap() as usize;
        let free_space = if i + 1 < chars.len() {
            chars[i + 1].to_digit(10).unwrap() as usize
        } else {
            0
        };

        for _ in 0..file_size {
            disk.push(id_counter);
        }

        if file_size > 0 {
            id_counter += 1;
        }

        for _ in 0..free_space {
            disk.push(-1);
        }
    }

    disk
}

fn compact_disk(disk: &mut Vec<i64>) {
    let mut free_segments = identify_free_segments(disk);
    let mut files = identify_files(disk);

    files.sort_by(|a, b| b.id.cmp(&a.id));

    for file in &files {
        if let Some((index, segment)) = free_segments
            .iter()
            .enumerate()
            .find(|(_, seg)| seg.size >= file.size && seg.start < file.start)
        {
            for i in 0..file.size {
                disk[segment.start + i] = file.id;
                disk[file.start + i] = -1;
            }

            free_segments[index].start += file.size;
            free_segments[index].size -= file.size;

            if free_segments[index].size == 0 {
                free_segments.remove(index);
            }

            free_segments.push(FreeSegment {
                start: file.start,
                size: file.size,
            });

            free_segments.sort_by(|a, b| a.start.cmp(&b.start));
        }
    }
}

fn identify_free_segments(disk: &[i64]) -> Vec<FreeSegment> {
    let mut segments = Vec::new();
    let mut start = None;

    for (i, &block) in disk.iter().enumerate() {
        if block == -1 {
            if start.is_none() {
                start = Some(i);
            }
        } else if let Some(start_index) = start {
            segments.push(FreeSegment {
                start: start_index,
                size: i - start_index,
            });
            start = None;
        }
    }

    if let Some(start_index) = start {
        segments.push(FreeSegment {
            start: start_index,
            size: disk.len() - start_index,
        });
    }

    segments
}

fn identify_files(disk: &[i64]) -> Vec<File> {
    let mut files = Vec::new();
    let mut start = None;
    let mut id = -1;

    for (i, &block) in disk.iter().enumerate() {
        if block >= 0 {
            if Some(block) != id.into() {
                if let Some(start_index) = start {
                    files.push(File {
                        id: id as i64,
                        start: start_index,
                        size: i - start_index,
                    });
                }
                start = Some(i);
                id = block;
            }
        } else if start.is_some() {
            files.push(File {
                id: id as i64,
                start: start.unwrap(),
                size: i - start.unwrap(),
            });
            start = None;
        }
    }

    if let Some(start_index) = start {
        files.push(File {
            id: id as i64,
            start: start_index,
            size: disk.len() - start_index,
        });
    }

    files
}


