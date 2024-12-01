use std::fs;
use std::path::Path;

fn main() {
    let content = fs::read_to_string(Path::new("./input_files/day_1.txt")).unwrap();
    let mut first_column = Vec::new();
    let mut second_column = Vec::new();
    for line in content.trim().lines() {
        let a: Vec<i32> = line.split("   ").map(|result| {
            result.parse::<i32>().unwrap()
        }).collect();
        first_column.push(a[0]);
        second_column.push(a[1]);
    }

    let distance = calculate_total_distance(first_column.clone(), second_column.clone());
    println!("Total distance: {}", distance);

    let similarity_score = calculate_similarity_score(&first_column, &second_column);
    println!("Similarity score: {}", similarity_score);
}

fn calculate_total_distance(mut first_column: Vec<i32>, mut second_column: Vec<i32>) -> i32 {
    first_column.sort();
    second_column.sort();

    let mut total_distance = 0;
    for i in 0..first_column.len() {
        total_distance += (second_column[i] - first_column[i]).abs()
    }

    total_distance
}

fn calculate_similarity_score(first_column: &Vec<i32>, second_column: &Vec<i32>) -> i32 {
    let mut similarity_score = 0;

    for first in first_column {
        let mut appearances = 0;
        for second in second_column {
            if first == second {
                appearances += 1;
            }
        }
        similarity_score += first * appearances;
    }
    similarity_score
}