use anyhow::{anyhow, bail, Result};
use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Untraveled,
    Traveled,
    Obstacle,
    Guard,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Tile::Untraveled => write!(f, "."),
            Tile::Traveled => write!(f, "X"),
            Tile::Obstacle => write!(f, "#"),
            Tile::Guard => write!(f, "^"),
        }
    }
}

struct Map {
    size: usize,
    storage: Vec<Tile>,
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let storage: Vec<Tile> = value
            .lines()
            .flat_map(|s| s.chars())
            .map(|c| match c {
                '.' => Tile::Untraveled,
                'X' => Tile::Traveled,
                '#' => Tile::Obstacle,
                '^' => Tile::Guard,
                _ => panic!("Unexpected character \"{}\" provided as input!", c),
            })
            .collect();

        let size = (storage.len() as f64).sqrt().round() as usize;

        Self { size, storage }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.size {
            for j in 0..self.size {
                write!(f, "{}", self.storage[i * self.size + j])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Map {
    fn try_get_tile(&self, i: usize, j: usize) -> Result<Tile> {
        if i >= self.size {
            bail!("Requested tile at position {}, max is {}", i, self.size);
        }

        if j >= self.size {
            bail!("Requested tile at position {}, max is {}", j, self.size);
        }

        Ok(self.storage[j * self.size + i])
    }

    fn try_set_tile(&mut self, i: usize, j: usize, tile: Tile) -> Result<()> {
        if i >= self.size {
            bail!("Requested tile at position {}, max is {}", i, self.size);
        }

        if j >= self.size {
            bail!("Requested tile at position {}, max is {}", j, self.size);
        }

        self.storage[j * self.size + i] = tile;

        Ok(())
    }

    fn iter(&self) -> std::slice::Iter<'_, Tile> {
        self.storage.iter()
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

struct Guard {
    map: Map,
    position: (usize, usize),
    direction: Direction,
}

impl Guard {
    fn new(map: Map) -> Self {
        let index = map
            .iter()
            .enumerate()
            .filter(|(_, &t)| t == Tile::Guard)
            .map(|(index, _)| index)
            .take(1)
            .next()
            .unwrap();

        let x = index % map.size;
        let y = (index - x) / map.size;

        Self {
            map,
            position: (x, y),
            direction: Direction::North,
        }
    }

    fn advance(&mut self) -> Result<()> {
        let next_position = match self.direction {
            Direction::North => (self.position.0, self.position.1.overflowing_sub(1).0),
            Direction::South => (self.position.0, self.position.1.overflowing_add(1).0),
            Direction::East => (self.position.0.overflowing_add(1).0, self.position.1),
            Direction::West => (self.position.0.overflowing_sub(1).0, self.position.1),
        };

        self.map
            .try_set_tile(self.position.0, self.position.1, Tile::Traveled)?;

        let next_tile = self.map.try_get_tile(next_position.0, next_position.1)?;

        if next_tile != Tile::Obstacle {
            self.position = next_position;
            self.map
                .try_set_tile(next_position.0, next_position.1, Tile::Guard)?;
            return Ok(());
        }

        // Only case to handle is obstacle
        // Only turn, do no movement
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        };

        Ok(())
    }
}

fn main() {
    let input = include_str!("../full_input.txt");

    let map = Map::from(input);

    let mut guard = Guard::new(map);

    println!("{}", guard.map);

    while let Ok(_) = guard.advance() {
        println!();
        // println!("{}", guard.map);
    }

    println!("{}", &guard.map);

    println!(
        "Number of Tiles: {}",
        guard.map.iter().filter(|&t| *t == Tile::Traveled).count()
    )
}
