use std::{collections::HashSet, convert::Infallible, num::ParseIntError, str::FromStr};

type Position = (i64, i64);

#[derive(Default, Debug)]
struct Rope {
    head: Position,
    tail: Position,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Movement {
    direction: Direction,
    distance: i64,
}

impl FromStr for Movement {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = s.split_once(" ").unwrap();

        Ok(Movement {
            direction: dir.parse().unwrap(),
            distance: dist.parse::<i64>()?,
        })
    }
}

impl Movement {
    fn into_steps(&self) -> Vec<Position> {
        match self.direction {
            Direction::Up => (0..self.distance).map(|_| (0, 1)).collect(),
            Direction::Down => (0..self.distance).map(|_| (0, -1)).collect(),
            Direction::Left => (0..self.distance).map(|_| (-1, 0)).collect(),
            Direction::Right => (0..self.distance).map(|_| (1, 0)).collect(),
        }
    }
}

impl Rope {
    fn apply_movement(self, movement: &Movement) -> (Rope, Vec<Position>) {
        dbg!(self.head, self.tail, movement);

        let mut visited: Vec<Position> = vec![];
        let rope_after_move = movement.into_steps().iter().inspect(|step| { dbg!(step); }).fold(self, |rope, head_delta| {
            let head = (rope.head.0 + head_delta.0, rope.head.1 + head_delta.1);
            let head_tail_distance = (head.0 - rope.tail.0, head.1 - rope.tail.1);

            let tail_delta = match head_tail_distance {
                (0, 2) => (0, 1),
                (0, -2) => (0, -1),
                (2, 0) => (1, 0),
                (-2, 0) => (-1, 0),
                (2, 1) => (1, 1),
                (2, -1) => (1, -1),
                (-2, 1) => (-1, 1),
                (-2, -1) => (-1, -1),
                (1, 2) => (1, 1),
                (1, -2) => (1, -1),
                (-1, 2) => (-1, 1),
                (-1, -2) => (-1, -1),
                _ => (0, 0),
            };

            dbg!(head, tail_delta);

            let tail = (rope.tail.0 + tail_delta.0, rope.tail.1 + tail_delta.1);

            visited.push(tail);

            Rope { head, tail }
        });

        (rope_after_move, visited)
    }
}

fn main() {
    let rope: Rope = Default::default();
    let moves: Vec<Movement> = include_str!("../input.txt")
        .lines()
        .flat_map(|line| line.parse::<Movement>())
        .collect();

    let mut tail_visited_fields = HashSet::<Position>::new();

    let _final_rope_position = moves.iter().fold(rope, |rope, movement| {
        let (rope_after_move, visited_in_move) = rope.apply_movement(movement);
        
        for m in visited_in_move { tail_visited_fields.insert(m); }

        rope_after_move
    });

    println!("part 1: {}", tail_visited_fields.len())
}
