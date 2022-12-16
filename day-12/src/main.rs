use std::{collections::{HashSet, VecDeque, HashMap}, vec};

#[derive(Debug, PartialEq)]
enum Mark {
    Start, // height 'a'/0
    End,   // height 'z'/26
    Height(u64),
}

impl Mark {
    fn get_height(&self) -> u64 {
        match self {
            Mark::Start => 0,
            Mark::End => 26,
            Mark::Height(h) => *h,
        }
    }
}

type Position = (u64, u64);

fn get_traversable_neighbours(heightmap: &Vec<Vec<Mark>>, pos: Position) -> Vec<Position> {
    let mark = &heightmap[pos.1 as usize][pos.0 as usize];
    let height = mark.get_height();

    vec![
        (pos.0, pos.1.saturating_sub(1)),
        (pos.0.saturating_sub(1), pos.1),
        (pos.0 + 1, pos.1),
        (pos.0, pos.1 + 1),
    ]
    .into_iter()
    .filter(|&(x, y)| { !(x == pos.0 && y == pos.1) })
    .filter(|(x, y)| {
        (*x > 0 || *y > 0) && *y < heightmap.len() as u64 && *x < heightmap[0].len() as u64
    })
    .filter(|(x, y)| {
        let mark_at_pos = &heightmap[*y as usize][*x as usize];
        height + 1 >= mark_at_pos.get_height()
    })
    .collect::<Vec<Position>>()
}

fn find_root_position(heightmap: &[Vec<Mark>]) -> Option<Position> {
    for (y, row) in heightmap.iter().enumerate() {
        for (x, mark) in row.iter().enumerate() {
            if mark == &Mark::Start { return Some((x as u64, y as u64)) }
        }
    }

    None
}

fn find_shortes_path(heightmap: &Vec<Vec<Mark>>) -> Option<Vec<Position>> {
    let root = find_root_position(heightmap).expect("start point should exist");
    let mut explored = HashSet::<Position>::from([root]);
    let mut queue: VecDeque<Position> = VecDeque::from([root]);
    let mut parent_map = HashMap::<Position, Option<Position>>::from([(root, None)]);

    while let Some(node) = queue.pop_front() {
        let current_mark = &heightmap[node.1 as usize][node.0 as usize];

        if current_mark == &Mark::End {
            let mut path: Vec<Option<Position>> = vec![];
            let mut current_node = Some(node);

            while let Some(n) = current_node {
                if let Some(p) = parent_map.get(&n) {
                    path.push(*p);
                    current_node = *p;
                }
            }

            return Some(path.iter().flat_map(|c| c.to_owned()).collect::<Vec<Position>>());
        }

        for neighbour in get_traversable_neighbours(heightmap, node).iter() {
            if !explored.contains(neighbour) {
                queue.push_back(*neighbour);
                parent_map.insert(*neighbour, Some(node));
                explored.insert(*neighbour);
            }
        }
    }

    None
}

fn main() {
    let heightmap = include_str!("../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'S' => Mark::Start,
                    'E' => Mark::End,
                    _ => Mark::Height(c as u64 - 97),
                })
                .collect::<Vec<Mark>>()
        })
        .collect::<Vec<_>>();

    let path = find_shortes_path(&heightmap).expect("path should not be empty");
    let steps = path.len();

    println!("{path:?}");
    println!("in {steps:?} steps");
}
