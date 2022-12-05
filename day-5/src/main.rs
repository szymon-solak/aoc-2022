use std::{str::FromStr, string::ParseError};

#[derive(Debug)]
struct Cargo {
    stacks: Vec<Vec<char>>,
}

#[derive(Debug)]
struct Step {
    from: u64,
    to: u64,
    amount: u64,
}

impl FromStr for Cargo {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let as_chunks = s.lines().map(|line| {
            line.chars()
                .collect::<Vec<char>>()
                .chunks(4) // "[C] "
                .map(|chunk| chunk.get(1).unwrap())
                .map(|maybe_char| {
                    if maybe_char == &' ' {
                        None
                    } else {
                        Some(maybe_char.to_owned())
                    }
                })
                .collect::<Vec<Option<char>>>()
        });

        let mut stacks: Vec<Vec<char>> = vec![];

        // without the label/number at the end
        for stack in as_chunks.rev().skip(1) {
            for (stack_number, stack_item) in stack.iter().enumerate() {
                if let Some(label) = stack_item {
                    if let Some(saved_stack) = stacks.get_mut(stack_number) {
                        saved_stack.push(label.to_owned());
                    } else {
                        stacks.insert(stack_number, vec![]);
                        stacks.get_mut(stack_number).unwrap().push(label.to_owned());
                    }
                }
            }
        }

        Ok(Cargo { stacks })
    }
}

impl FromStr for Step {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut as_parts = s.split_whitespace();
        let (_, amount, _, from, _, to) = (
            as_parts.next(),
            as_parts.next(),
            as_parts.next(),
            as_parts.next(),
            as_parts.next(),
            as_parts.next(),
        );

        Ok(Step {
            amount: amount.unwrap().parse::<u64>()?,
            from: from.unwrap().parse::<u64>()? - 1, // as index
            to: to.unwrap().parse::<u64>()? - 1, // as index
        })
    }
}

impl Cargo {
    fn apply(&mut self, step: &Step) -> &mut Cargo {
        let crates = self.stacks
            .get_mut(step.from as usize)
            .unwrap();

        let crates_to_move = crates
            .drain(crates.len() - step.amount as usize..).collect::<Vec<char>>();

        for c in crates_to_move {
            self.stacks.get_mut(step.to as usize).unwrap().push(c);
        }
        self
    }

    fn get_top_crates(&self) -> Vec<char> {
        self.stacks.iter().filter_map(|stack| { stack.last() }).map(|c| c.to_owned()).collect()
    }
}

fn get_crates_on_top(initial_state: &str, steps: &str) -> Vec<char> {
    let mut cargo = initial_state.parse::<Cargo>().unwrap();

    let steps = steps.lines().map(|step| step.parse::<Step>().unwrap());
    let final_state = steps.fold(&mut cargo, |state, step| {state.apply(&step)});

    final_state.get_top_crates()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (initial_state, steps) = input.split_once("\n\n").unwrap();
    let crates_on_top = get_crates_on_top(initial_state, steps).iter().collect::<String>();

    println!("{crates_on_top:?}");
}

#[cfg(test)]
mod tests {
    use crate::{get_crates_on_top};

    #[test]
    fn example_case() {
        // given
        let initial_state = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3";

        let steps = "
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2".trim_start();

        // when
        let crates_on_top = get_crates_on_top(initial_state, steps).iter().collect::<String>();

        // then
        assert_eq!(crates_on_top, "MCD")
    }
}
