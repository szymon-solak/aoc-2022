fn to_priority(ch: char) -> u64 {
    if ch.is_ascii_lowercase() {
        (ch as u8 - 96).into()
    } else {
        (ch as u8 - 38).into()
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let rucksacks: Vec<&str> = input
        .lines()
        .collect();

    let groups = rucksacks
        .chunks(3)
        .map(|elf_group| {
            let elf_a = elf_group.get(0).unwrap();
            let elf_b = elf_group.get(1).unwrap();
            let elf_c = elf_group.get(2).unwrap();

            for ch in elf_a.chars() {
                if elf_b.contains(ch) && elf_c.contains(ch) { return ch }
            }

            unreachable!()
        })
        .map(|ch| to_priority(ch));

    println!("sum of priorities: {}", groups.sum::<u64>())
}
