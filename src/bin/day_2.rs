use std::fs;
use std::path::Path;

fn main() {
    let content = fs::read_to_string(Path::new("./input_files/day_2.txt")).unwrap();
    let mut rows = Vec::new();
    for line in content.trim().lines() {
        let levels: Vec<i32> = line.trim().split_whitespace().map(|result| {
            result.parse::<i32>().unwrap()
        }).collect();
        rows.push(levels);
    }

    let safe_rows_count = count_safe_rows(&rows);
    println!("Safe rows: {}", safe_rows_count);

    let safe_dampened_rows_count = count_dampened_safe_rows(&rows);
    println!("Safe dampened rows: {}", safe_dampened_rows_count);
}

fn count_safe_rows(rows: &Vec<Vec<i32>>) -> u32 {
    let mut safe_rows = 0;
    for row in rows {
        if are_levels_safe(&row) {
            safe_rows += 1;
        }
    }
    safe_rows
}

fn count_dampened_safe_rows(rows: &Vec<Vec<i32>>) -> u32 {
    let mut safe_rows = 0;
    for row in rows {
        if are_dampened_levels_safe(&row) {
            safe_rows += 1;
        }
    }
    safe_rows
}

fn are_dampened_levels_safe(levels: &Vec<i32>)-> bool {
    if are_levels_safe(&levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut modified_levels = Vec::new();
        for (j, level) in levels.iter().enumerate() {
            if i != j {
                modified_levels.push(level.clone());
            }
        }
        if are_levels_safe(&modified_levels) {
            return true;
        }
    }

    false
}

fn are_levels_safe(levels: &Vec<i32>) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let is_increasing = match levels[1] - levels[0] {
        1..=3 => true,
        -3..=-1 => false,
        _ => {
            return false;
        }
    };

    for i in 1..levels.len() - 1 {
        let first_level = levels[i];
        let second_level = levels[i + 1];
        let difference = second_level - first_level;
        if is_increasing && (difference < 1 || difference > 3) {
            return false;
        }
        if !is_increasing && (difference > -1 || difference < -3) {
            return false;
        }
    }

    true
}