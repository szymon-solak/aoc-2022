use std::{collections::HashSet, convert::Infallible, num::ParseIntError, str::FromStr};

type Position = (i64, i64);

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

struct MultiKnotRope {
    knots: Vec<Position>,
}

impl MultiKnotRope {
    fn adjust_tail(head: Position, tail: Position) -> Position {
        let head_tail_distance = (head.0 - tail.0, head.1 - tail.1);

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
            (-2, -2) => (-1, -1),
            (2, 2) => (1, 1),
            (-2, 2) => (-1, 1),
            (2, -2) => (1, -1),
            _ => (0, 0),
        };

        (tail.0 + tail_delta.0, tail.1 + tail_delta.1)
    }

    fn apply_movement(self, movement: &Movement) -> (MultiKnotRope, Vec<Position>) {
        let mut visited: Vec<Position> = vec![];

        let rope_after_move = movement
            .into_steps()
            .iter()
            .fold(self, |rope, step| {
                let head = rope.knots.get(0).unwrap();
                let mut knots: Vec<Position> = vec![
                    (head.0 + step.0, head.1 + step.1)
                ];

                for knot in rope.knots.iter().skip(1) {
                   knots.push(
                        MultiKnotRope::adjust_tail(
                            *knots.last().unwrap(),
                            *knot,
                        )
                    ) 
                }

                visited.push(*knots.last().unwrap());

                MultiKnotRope { knots }
            });

        (rope_after_move, visited)
    }
}

fn print_rope(rope: &MultiKnotRope) {
    print!("{}[2J", 27 as char); // clear
    
    for y in (-64..64).rev() {
        let mut line: String = "".to_string();

        for x in -64..64 {
            if let Some(_) = rope.knots.iter().find(|k| k.0 == x && k.1 == y) {
                line.push_str("#");
            } else {
                line.push_str(".");
            }
        }

        println!("{line}");
    } 
}

fn main() {
    let rope = MultiKnotRope {
        knots: vec![(0, 0)].repeat(10),
    };
    let moves: Vec<Movement> = include_str!("../input.txt")
        .lines()
        .flat_map(|line| line.parse::<Movement>())
        .collect();

    let mut tail_visited_fields = HashSet::<Position>::new();

    let _final_rope_position = moves.iter().fold(rope, |rope, movement| {
        let (rope_after_move, visited_in_move) = rope.apply_movement(movement);

        // print_rope(&rope_after_move);

        for m in visited_in_move {
            tail_visited_fields.insert(m);
        }

        rope_after_move
    });
    
    println!("part 2: {:?}", tail_visited_fields.len());
}
