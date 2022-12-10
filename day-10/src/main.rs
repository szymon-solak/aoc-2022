use std::{num::ParseIntError, str::FromStr};

enum Instruction {
    Noop,
    Addx(i64),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().collect::<Vec<&str>>().as_slice() {
            ["noop"] => Ok(Instruction::Noop),
            ["addx", val] => Ok(Instruction::Addx(val.parse::<i64>().unwrap())),
            _ => unreachable!(),
        }
    }
}

fn draw_crt(register_states: &Vec<i64>) {
    for y in 0..6 {
        for x in 0..40 {
            let crt_position: i64 = x + (y * 40);
            let sprite_center: i64 = register_states.iter().take(crt_position as usize + 1).sum();

            if (sprite_center - 1..=sprite_center + 1).contains(&(crt_position % 40)) {
                print!("#")
            } else {
                print!(".")
            }
        }

        println!("");
    }
}

fn main() {
    let mut register_states: Vec<i64> = vec![1];

    for instruction in include_str!("../input.txt")
        .lines()
        .filter_map(|line| line.parse::<Instruction>().ok())
    {
        match instruction {
            Instruction::Noop => register_states.push(0),
            Instruction::Addx(val) => {
                register_states.push(0);
                register_states.push(val);
            }
        }
    }

    let interesting_signals = vec![20, 60, 100, 140, 180, 220];
    let signal_sum = interesting_signals
        .iter()
        .map(|&tick| tick as i64 * register_states.iter().take(tick).sum::<i64>())
        .inspect(|signal| {
            dbg!(signal);
        })
        .sum::<i64>();

    println!("{signal_sum}");

    draw_crt(&register_states);
}
