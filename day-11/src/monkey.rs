use std::{ops::Rem, str::FromStr};

#[derive(Debug, Clone)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

#[derive(Debug, Clone)]
pub(crate) struct Monkey {
    pub items: Vec<u64>,
    pub inspection_count: u64,
    operation: Operation,
    test: u64,
    throw_to_if_sucess: u64,
    throw_to_if_failure: u64,
}

impl Monkey {
    pub fn process_item(&self, worry_level: u64) -> (u64, u64) {
        let after_op = match self.operation {
            Operation::Add(n) => worry_level + n,
            Operation::Multiply(n) => worry_level * n,
            Operation::Square => worry_level * worry_level,
        };

        let after_div = after_op.div_floor(3);
        let test_successful = after_div.rem(self.test) == 0;

        if test_successful {
            (self.throw_to_if_sucess, after_div)
        } else {
            (self.throw_to_if_failure, after_div)
        }
    }
}

impl FromStr for Monkey {
    type Err = color_eyre::eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .skip(1)
            .map(|line| line.trim())
            .collect::<Vec<&str>>();

        let starting_items = lines
            .get(0)
            .unwrap()
            .split_once(":")
            .unwrap()
            .1
            .split(",")
            .filter_map(|d| d.trim().parse::<u64>().ok())
            .collect::<Vec<u64>>();

        let operation = match lines
            .get(1)
            .unwrap()
            .split_once("=")
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .as_slice()
        {
            ["old", "*", "old"] => Operation::Square,
            ["old", "+", n] => Operation::Add(n.parse::<u64>().unwrap()),
            ["old", "*", n] => Operation::Multiply(n.parse::<u64>().unwrap()),
            _ => unreachable!(),
        };

        let test = lines
            .get(2)
            .unwrap()
            .split_once("divisible by ")
            .unwrap()
            .1
            .parse::<u64>();
        let throw_to_if_sucess = lines
            .get(3)
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .parse::<u64>();
        let throw_to_if_failure = lines
            .get(4)
            .unwrap()
            .split_once("monkey ")
            .unwrap()
            .1
            .parse::<u64>();

        Ok(Monkey {
            items: starting_items,
            operation,
            test: test?,
            throw_to_if_sucess: throw_to_if_sucess?,
            throw_to_if_failure: throw_to_if_failure?,
            inspection_count: 0,
        })
    }
}
