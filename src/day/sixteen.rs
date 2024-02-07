use std::usize;

pub fn main() {
    let day = "sixteen";
    let answer =
        part_two(std::fs::read_to_string("inputs/".to_owned() + day + "/small.txt").unwrap());
    assert_eq!(answer, 51);

    let answer =
        part_two(std::fs::read_to_string("inputs/".to_owned() + day + "/big.txt").unwrap());
    dbg!(answer);
}

fn part_one(content: String) -> usize {
    let mut contraption: Vec<Vec<_>> = content
        .lines()
        .map(|line| line.chars().map(|char| Tile::new(char)).collect())
        .collect();

    next_tile(-1, 0, Direction::East, &mut contraption);

    contraption
        .iter()
        .flatten()
        .map(|tile| if tile.energized { 1 } else { 0 })
        .sum()
}

fn part_two(content: String) -> usize {
    let mut contraption: Vec<Vec<_>> = content
        .lines()
        .map(|line| line.chars().map(|char| Tile::new(char)).collect())
        .collect();

    let hori = (0..contraption.len())
        .map(|y| {
            let y = y as isize;
            let mut east = contraption.clone();
            next_tile(-1, y, Direction::East, &mut east);

            let mut west = contraption.clone();
            next_tile(contraption[0].len() as isize, y, Direction::West, &mut west);

            std::cmp::max(find_energy_level(&east), find_energy_level(&west))
        })
        .max()
        .unwrap();

    let vert = (0..contraption[0].len())
        .map(|x| {
            let x = x as isize;
            let mut north = contraption.clone();
            next_tile(x, contraption.len() as isize, Direction::North, &mut north);

            let mut south = contraption.clone();
            next_tile(x, -1, Direction::South, &mut south);

            std::cmp::max(find_energy_level(&north), find_energy_level(&south))
        })
        .max()
        .unwrap();
    std::cmp::max(hori, vert)
}

fn find_energy_level(contraption: &Vec<Vec<Tile>>) -> usize {
    contraption
        .iter()
        .flatten()
        .map(|tile| if tile.energized { 1 } else { 0 })
        .sum()
}

fn next_tile(x: isize, y: isize, direction: Direction, contraption: &mut Vec<Vec<Tile>>) {
    //hit a wall
    let (x, y) = match direction {
        Direction::North => (x, y - 1),
        Direction::South => (x, y + 1),
        Direction::East => (x + 1, y),
        Direction::West => (x - 1, y),
    };

    if x < 0 || x >= contraption[0].len() as isize || y < 0 || y >= contraption.len() as isize {
        return {};
    }

    let tile = &mut contraption[y as usize][x as usize];
    if tile.has_been_entered_going(direction) {
        return {};
    }

    tile.enter_from(direction);
    match tile.symbol {
        '/' => match direction {
            Direction::North => {
                next_tile(x, y, Direction::East, contraption);
            }
            Direction::South => {
                next_tile(x, y, Direction::West, contraption);
            }
            Direction::East => {
                next_tile(x, y, Direction::North, contraption);
            }
            Direction::West => {
                next_tile(x, y, Direction::South, contraption);
            }
        },
        '\\' => match direction {
            Direction::North => {
                next_tile(x, y, Direction::West, contraption);
            }
            Direction::South => {
                next_tile(x, y, Direction::East, contraption);
            }
            Direction::East => {
                next_tile(x, y, Direction::South, contraption);
            }
            Direction::West => {
                next_tile(x, y, Direction::North, contraption);
            }
        },
        '-' => match direction {
            Direction::North | Direction::South => {
                next_tile(x, y, Direction::East, contraption);
                next_tile(x, y, Direction::West, contraption);
            }
            Direction::East | Direction::West => {
                next_tile(x, y, direction, contraption);
            }
        },
        '|' => match direction {
            Direction::North | Direction::South => {
                next_tile(x, y, direction, contraption);
            }
            Direction::East | Direction::West => {
                next_tile(x, y, Direction::North, contraption);
                next_tile(x, y, Direction::South, contraption);
            }
        },
        _ => {
            next_tile(x, y, direction, contraption);
        }
    };
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    symbol: char,
    energized: bool,
    north: bool,
    south: bool,
    east: bool,
    west: bool,
}

impl Tile {
    fn new(symbol: char) -> Self {
        Tile {
            symbol,
            energized: false,
            north: false,
            south: false,
            east: false,
            west: false,
        }
    }

    fn has_been_entered_going(&mut self, direction: Direction) -> bool {
        match direction {
            Direction::North => self.north,
            Direction::South => self.south,
            Direction::East => self.east,
            Direction::West => self.west,
        }
    }

    fn enter_from(&mut self, direction: Direction) {
        self.energized = true;
        match direction {
            Direction::North => self.north = true,
            Direction::South => self.south = true,
            Direction::East => self.east = true,
            Direction::West => self.west = true,
        }
    }
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}
