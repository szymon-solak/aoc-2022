#![feature(int_roundings)]

mod monkey;

use monkey::Monkey;

fn process_round(monkeys: &mut [Monkey]) {
    for monkey_index in 0..monkeys.len() {
       let monkey = &mut monkeys[monkey_index].clone();

       for worry_level in monkey.items.iter().copied() {
            let (receiver, new_worry_level) = monkey.process_item(worry_level);
            monkeys[receiver as usize].items.push(new_worry_level);
        } 

        monkeys[monkey_index].inspection_count += monkey.items.len() as u64;
        monkeys[monkey_index].items.clear();
    }
}

fn main() -> color_eyre::eyre::Result<(), color_eyre::eyre::Report> {
    color_eyre::install()?;

    let mut monkeys: Vec<Monkey> = include_str!("../input.txt")
        .split("\n\n")
        .filter_map(|segment| segment.parse::<Monkey>().ok())
        .collect();

    for _round in 0..20 {
        process_round(&mut monkeys);
    }

    let mut monkey_business_level: Vec<u64> = monkeys
        .into_iter()
        .map(|m| m.inspection_count)
        .collect();
    monkey_business_level.sort_unstable_by_key(|&count| std::cmp::Reverse(count));
    let monkey_business_level: u64 = monkey_business_level.iter().take(2).product();

    println!("{monkey_business_level:?}");

    Ok(())
}
