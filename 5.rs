use std::collections::HashMap;
use std::fs;
use std::io;

fn read_input() -> io::Result<(HashMap<i64, Vec<i64>>, Vec<Vec<i64>>)> {
    let file_path = "5_input.txt";

    let content = fs::read_to_string(file_path)?;

    let sections: Vec<&str> = content.split("\n\n").collect();

    let mut map: HashMap<i64, Vec<i64>> = HashMap::new();

    for line in sections[0].lines() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 2 {
            let key: i64 = parts[0].trim().parse().expect("cannot parse first number");
            let value: i64 = parts[1].trim().parse().expect("cannot parse second number");
            map.entry(key).or_insert_with(Vec::new).push(value);
        }
    }

    let mut orders: Vec<Vec<i64>> = Vec::new();
    for line in sections[1].lines() {
        let values: Vec<i64> = line
            .split(',')
            .map(|v| v.trim().parse().expect("cannot parse value"))
            .collect();
        orders.push(values);
    }

    Ok((map, orders))
}

fn find_middle(update: &[i64]) -> i64 {
    let middle_index = update.len() / 2;
    update[middle_index]
}

fn process_order(order_map: &HashMap<i64, Vec<i64>>, order: &mut Vec<i64>, fix_order: bool) -> bool {
    let mut valid = true;

    for i in 0..order.len() {
        for j in i + 1..order.len() {
            if let Some(after_pages) = order_map.get(&order[j]) {
                if after_pages.contains(&order[i]) {
                    valid = false;
                    if fix_order {
                        order.swap(i, j);
                    }
                }
            }
        }
    }

    valid
}


fn main() {
    let (order_map, mut orders) = read_input().expect("failed to read input");

    let mut sum_correct: i64 = 0;
    let mut sum_wrong: i64 = 0;

    for order in &mut orders {
        if process_order(&order_map, order, false) {
            let middle_page = find_middle(order);
            sum_correct += middle_page;
        } else {
            process_order(&order_map, order, true);
            let middle_page = find_middle(order);
            sum_wrong += middle_page;
        }
    }

    println!("{:?}", sum_correct); 

    //2
    println!("{:?}", sum_wrong); 
}