fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let total_points = input
        .lines()
        .map(|l| {
            let moves: Vec<&str> = l.split_whitespace().take(2).collect();
            let opponent = moves.get(0).unwrap().chars().next().unwrap();
            let you = moves.get(1).unwrap().chars().next().unwrap();

            match opponent {
                'A' => {
                    // Rock
                    match you {
                        'X' => 3 + 0,
                        'Y' => 1 + 3,
                        'Z' => 2 + 6,
                        _ => unreachable!(),
                    }
                }
                'B' => {
                    // Paper
                    match you {
                        'X' => 1 + 0,
                        'Y' => 2 + 3,
                        'Z' => 3 + 6,
                        _ => unreachable!(),
                    }
                }
                'C' => {
                    // Scissors
                    match you {
                        'X' => 2 + 0,
                        'Y' => 3 + 3,
                        'Z' => 1 + 6,
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        })
        .sum::<i64>();

    println!("total points: {total_points:?}")
}
