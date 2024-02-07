use std::usize;

pub fn main() {
    let day = "seventeen";
    let answer =
        part_one(std::fs::read_to_string("inputs/".to_owned() + day + "/small.txt").unwrap());
    assert_eq!(answer, 102);

    let answer =
        part_one(std::fs::read_to_string("inputs/".to_owned() + day + "/big.txt").unwrap());
    dbg!(answer);
}

fn part_one(content: String) -> u16 {
    let mut contraption: Vec<Vec<_>> = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| Tile::new(char.to_digit(10).unwrap() as u16))
                .collect()
        })
        .collect();

    next_tile(0, 0, 0, Direction::East(1), &mut contraption);
    next_tile(0, 0, 0, Direction::South(1), &mut contraption);
    let end = &contraption[contraption.len() - 1][contraption[0].len() - 1];
    std::cmp::min(
        *end.north.iter().min().unwrap(),
        std::cmp::min(
            *end.south.iter().min().unwrap(),
            std::cmp::min(
                *end.east.iter().min().unwrap(),
                *end.west.iter().min().unwrap(),
            ),
        ),
    )
}

fn next_tile(
    x: isize,
    y: isize,
    heat: u16,
    direction: Direction,
    contraption: &mut Vec<Vec<Tile>>,
) {
    //get new coord
    let (x, y) = match direction {
        Direction::North(_) => (x, y - 1),
        Direction::South(_) => (x, y + 1),
        Direction::East(_) => (x + 1, y),
        Direction::West(_) => (x - 1, y),
    };

    //hit a wall
    if x < 0 || x >= contraption[0].len() as isize || y < 0 || y >= contraption.len() as isize {
        return {};
    }

    let tile = &mut contraption[y as usize][x as usize];
    let heat = heat + tile.heat;

    if tile.has_entered(&direction) < heat {
        return {};
    }

    tile.enter(&direction, heat);

    match direction {
        Direction::North(count) => {
            next_tile(x, y, heat, Direction::East(1), contraption);
            next_tile(x, y, heat, Direction::West(1), contraption);
            if count < 3 {
                next_tile(x, y, heat, Direction::North(count + 1), contraption);
            }
        }
        Direction::South(count) => {
            next_tile(x, y, heat, Direction::East(1), contraption);
            next_tile(x, y, heat, Direction::West(1), contraption);
            if count < 3 {
                next_tile(x, y, heat, Direction::South(count + 1), contraption);
            }
        }
        Direction::East(count) => {
            next_tile(x, y, heat, Direction::North(1), contraption);
            next_tile(x, y, heat, Direction::South(1), contraption);
            if count < 3 {
                next_tile(x, y, heat, Direction::East(count + 1), contraption);
            }
        }
        Direction::West(count) => {
            next_tile(x, y, heat, Direction::North(1), contraption);
            next_tile(x, y, heat, Direction::South(1), contraption);
            if count < 3 {
                next_tile(x, y, heat, Direction::West(count + 1), contraption);
            }
        }
    };
}

#[derive(Debug)]
struct Tile {
    heat: u16,
    north: Vec<u16>,
    south: Vec<u16>,
    east: Vec<u16>,
    west: Vec<u16>,
}

impl Tile {
    fn new(heat: u16) -> Self {
        Tile {
            heat,
            north: [u16::MAX, u16::MAX, u16::MAX].to_vec(),
            south: [u16::MAX, u16::MAX, u16::MAX].to_vec(),
            east: [u16::MAX, u16::MAX, u16::MAX].to_vec(),
            west: [u16::MAX, u16::MAX, u16::MAX].to_vec(),
        }
    }

    fn has_entered(&mut self, direction: &Direction) -> u16 {
        match direction {
            Direction::North(count) => self.north[*count as usize - 1],
            Direction::South(count) => self.south[*count as usize - 1],
            Direction::East(count) => self.east[*count as usize - 1],
            Direction::West(count) => self.west[*count as usize - 1],
        }
    }

    fn enter(&mut self, direction: &Direction, path: u16) {
        match direction {
            Direction::North(count) => self.north[*count as usize - 1] = path,
            Direction::South(count) => self.south[*count as usize - 1] = path,
            Direction::East(count) => self.east[*count as usize - 1] = path,
            Direction::West(count) => self.west[*count as usize - 1] = path,
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    North(u8),
    South(u8),
    East(u8),
    West(u8),
}
