use std::{path::Iter, task::Context, usize};

pub fn main() {
    let answer = part_one(std::fs::read_to_string("inputs/fourteen_small.txt").unwrap());
    assert_eq!(answer, 64);

    let answer = part_one(std::fs::read_to_string("inputs/fourteen_big.txt").unwrap());
    dbg!(answer);
}

fn part_one(content: String) -> usize {
    let mut content = content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut copy = content.clone();

    tilt(&mut content);
    tilt(&mut copy);
    tilt(&mut copy);

    while copy != content {
        tilt(&mut content);
        tilt(&mut copy);
        tilt(&mut copy);
    }

    let mut count = 1;
    tilt(&mut content);
    while (copy != content) {
        tilt(&mut content);
        count += 1;
    }

    let more = 1000000000 % count;

    for _ in 0..more {
        tilt(&mut content);
    }
    print_pretty(content.to_vec());

    find_load(content)
}

fn print_pretty(data: Vec<Vec<char>>) {
    let data = data
        .iter()
        .map(|line| line.iter().collect::<String>() + "\r\n")
        .collect::<String>();
    println!("{}", data);
}

fn tilt(data: &mut Vec<Vec<char>>) {
    let height = data.len();
    let width = data[0].len();

    //tilt north
    for column in 0..width {
        let mut stop = 0;
        for row in 0..height {
            match data[row][column] {
                '#' => {
                    stop = row + 1;
                }
                'O' => {
                    data[row][column] = '.';
                    data[stop][column] = 'O';
                    stop += 1;
                }
                _ => {}
            }
        }
    }
    // print_pretty(data.to_vec());

    //tilt west
    for row in 0..height {
        let mut stop = 0;
        for column in 0..width {
            match data[row][column] {
                '#' => {
                    stop = column + 1;
                }
                'O' => {
                    data[row][column] = '.';
                    data[row][stop] = 'O';
                    stop += 1;
                }
                _ => {}
            }
        }
    }
    // print_pretty(data.to_vec());

    // tilt south
    for column in 0..width {
        let mut stop = height;
        for row in (0..height).rev() {
            match data[row][column] {
                '#' => {
                    stop = row;
                }
                'O' => {
                    data[row][column] = '.';
                    data[stop - 1][column] = 'O';
                    stop -= 1;
                }
                _ => {}
            }
        }
    }
    // print_pretty(data.to_vec());

    // tilt east
    for row in 0..height {
        let mut stop = width;
        for column in (0..width).rev() {
            match data[row][column] {
                '#' => {
                    stop = column;
                }
                'O' => {
                    data[row][column] = '.';
                    data[row][stop - 1] = 'O';
                    stop -= 1;
                }
                _ => {}
            }
        }
    }
    // print_pretty(data.to_vec());
}

fn find_load(data: Vec<Vec<char>>) -> usize {
    let height = data.len();
    let width = data[0].len();

    let mut load = 0;

    for column in 0..width {
        for row in 0..height {
            match data[row][column] {
                'O' => {
                    load += height - row;
                }
                _ => {}
            }
        }
    }

    load
}
