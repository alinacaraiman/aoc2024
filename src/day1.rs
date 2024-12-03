use std::iter::zip;

use crate::misc::read_lines;

fn part1(first_list: &[i64], second_list: &[i64]) {
    let mut final_list: Vec<i64> = Vec::new();
    for (x, y) in zip(first_list, second_list) {
        let result = (x - y).abs();
        final_list.push(result);
    }

    let location: i64 = final_list.iter().sum();
    println!("Day 1 Part 1: Location {}", location)
}

fn part2(first_list: &[i64], second_list: &[i64]) {
    let mut final_list: Vec<usize> = Vec::new();
    for x in first_list {
        let count = second_list.iter().filter(|&n| n == x).count() * ((*x) as usize);
        final_list.push(count);
    }
    let location: usize = final_list.iter().sum();
    println!("Day 1 Part 2: Location {}", location)
}

pub fn solve() {
    let mut first_list: Vec<i64> = Vec::new();
    let mut second_list: Vec<i64> = Vec::new();
    if let Ok(lines) = read_lines("inputs/day1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let splitted: Vec<&str> = line.split("   ").collect();
            first_list.push(splitted[0].parse::<i64>().unwrap());
            second_list.push(splitted[1].parse::<i64>().unwrap());
        }
    }
    first_list.sort();
    second_list.sort();

    part1(&first_list, &second_list);
    part2(&first_list, &second_list);
}
