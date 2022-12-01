fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut by_calories: Vec<i64> = input
        .split("\n\n")
        .map(|group_as_string| {
            group_as_string.lines().map(|l| l.parse::<i64>().unwrap()).sum()
        })
        .collect();

    by_calories.sort();

    let sum_of_top_three = by_calories.iter().rev().take(3).sum::<i64>();

    println!("Most calories: {:?}", sum_of_top_three);
}
