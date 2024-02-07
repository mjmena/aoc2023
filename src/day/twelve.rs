use core::hash;
use std::{char, cmp::min, io::repeat};

pub fn main() {
    // let answer = part_one(String::from("?#?#?#?#?#?#?#? 1,3,1,6"));
    // assert_eq!(answer, 4);

    let answer = part_one(std::fs::read_to_string("inputs/twelve_small.txt").unwrap());
    assert_eq!(answer, 525152);

    let answer = part_one(std::fs::read_to_string("inputs/twelve_big.txt").unwrap());
    dbg!(answer);
}

fn part_one(content: String) -> usize {
    content
        .lines()
        .map(|line| {
            let (data, groups) = line.split_once(" ").unwrap();
            let data = (data.to_string() + "?").repeat(5);
            let data = data[0..data.chars().count() - 1]
                .chars()
                .collect::<Vec<_>>();
            let groups = groups.split(",").map(|num| num.parse().unwrap());
            let groups = groups
                .clone()
                .chain(groups.clone())
                .chain(groups.clone())
                .chain(groups.clone())
                .chain(groups)
                .collect::<Vec<_>>();
            find_arrangements(&data, &groups)
        })
        .sum()
}

fn find_arrangements(data: &[char], groups: &[usize]) -> usize {
    let Some(group) = groups.first() else {
        panic!("no group");
    };

    //how much space we need after
    let hash_index = match data.iter().enumerate().find(|(_, c)| **c == '#') {
        Some(hash) => hash.0 + 1,
        None => data.len(),
    };

    let needed_spaces = groups.iter().sum::<usize>() + groups.len() - 2;

    (0..min(data.len() - needed_spaces, hash_index))
        .map(|check_position| {
            if data[check_position..(check_position + group)].contains(&'.') {
                0
            } else {
                if groups.len() == 1 {
                    if data[check_position + group..].contains(&'#') {
                        0
                    } else {
                        1
                    }
                } else {
                    if data[check_position + group] == '#' {
                        0
                    } else {
                        find_arrangements(&data[check_position + group + 1..], &groups[1..])
                    }
                }
            }
        })
        .sum()
}
