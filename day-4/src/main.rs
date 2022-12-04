use std::ops::Range;

fn parse_to_range(input: &str) -> Range<u64> {
    let (start, end) = input.split_once("-").unwrap();

    println!("parse_to_range {start} {end}");

    Range { start: start.parse::<u64>().unwrap() , end: end.parse::<u64>().unwrap() }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let pairs = input
        .lines()
        .map(|line| {
            let (l, r) = line.split_once(",").unwrap();
            
            (parse_to_range(l), parse_to_range(r))
        })
        .collect::<Vec<(Range<u64>, Range<u64>)>>();

    let fully_overlapping_paris_count = pairs
        .iter()
        .filter(|(l ,r)| {
            if l.start <= r.start && l.end >= r.end { return true }
            if r.start <= l.start && r.end >= l.end { return true }

            return false
        })
        .count();

    println!("fully contained pairs: {fully_overlapping_paris_count}")
}
