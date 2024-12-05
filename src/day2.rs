use crate::misc::read_lines;

fn part1(report: &[i32]) -> bool {
    let increasing = report.windows(2).all(|pair| {
        let diff = pair[1] - pair[0];
        (1..=3).contains(&diff)
    });

    let decreasing = report.windows(2).all(|pair| {
        let diff = pair[0] - pair[1];
        (1..=3).contains(&diff)
    });

    increasing || decreasing
}

fn part2(report: &[i32]) -> bool {
    if part1(report) {
        return true;
    }

    for i in 0..report.len() {
        let fixed_report: Vec<i32> = report
            .iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .map(|(_, &val)| val)
            .collect();

        if part1(&fixed_report) {
            return true;
        }
    }

    false
}

pub fn solve() {
    let mut levels: Vec<Vec<i32>> = Vec::new();
    if let Ok(lines) = read_lines("inputs/day2.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let report: Vec<i32> = line
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            levels.push(report);
        }
    }
    let safe_count_part1 = levels.iter().filter(|report| part1(report)).count();
    let safe_count_part2 = levels
        .iter()
        .filter(|report| part2(report))
        .count();

    println!("Day 2 Part 1: {}", safe_count_part1);
    println!("Day 2 Part 2: {}", safe_count_part2);
}
