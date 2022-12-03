fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let priorities = input
        .lines()
        .map(|l| {
            let (l, r) = l.split_at(l.len() / 2);

            for ch in l.chars() {
                if r.contains(ch) { return ch; }
            }

            unreachable!()
        })
        .map(|ch| {
            if ch.is_ascii_lowercase() {
                ((ch as u8) - 96) as u64
            } else {
                ((ch as u8) - 38) as u64
            }
        });

    println!("sum of priorities: {}", priorities.sum::<u64>())
}
