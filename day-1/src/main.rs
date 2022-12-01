fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let most_calories: Option<i64> = input
        .split("\n\n")
        .map(|group_as_string| {
            group_as_string.lines().map(|l| l.parse::<i64>().unwrap()).sum()
        })
        .max();

    println!("Most calories: {:?}", most_calories);
}
