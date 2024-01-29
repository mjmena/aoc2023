#![feature(iter_map_windows)]
fn main() {
    let file = std::fs::read_to_string("day_nine_input.txt").unwrap();
    let patterns = file.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    });
    dbg!(patterns.map(derive).sum::<i64>());
}

fn derive(pattern: Vec<i64>) -> i64 {
    if pattern.clone().into_iter().all(|p| p == 0) {
        return 0;
    }
    pattern.clone().into_iter().nth(0).unwrap()
        - derive(
            pattern
                .clone()
                .into_iter()
                .map_windows(|[a, b]| b - a)
                .collect::<Vec<i64>>(),
        )
}

fn derive_part_one(pattern: Vec<i64>) -> i64 {
    if pattern.clone().into_iter().all(|p| p == 0) {
        return 0;
    }
    derive_part_one(
        pattern
            .clone()
            .into_iter()
            .map_windows(|[a, b]| b - a)
            .collect::<Vec<i64>>(),
    ) + pattern.clone().into_iter().last().unwrap()
}

fn part_one() {
    let file = std::fs::read_to_string("day_nine_input.txt").unwrap();
    let patterns = file.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    });
    dbg!(patterns.map(derive_part_one).sum::<i64>());
}
