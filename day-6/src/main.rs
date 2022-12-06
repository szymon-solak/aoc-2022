fn find_marker<'a>(packet: &'a Vec<char>, unique_chars: usize) -> Option<(usize, &'a[char])> {
    packet.windows(unique_chars).enumerate().find(|(_, chars)| {
        let mut unique = chars.to_vec();
        unique.sort_unstable();
        unique.dedup();

        unique.len() == unique_chars
    })
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    if let Some((pos, _)) = find_marker(&input, 4) {
        let start_of_packet_marker = pos + 4;
        println!("{start_of_packet_marker}")
    }

    if let Some((pos, _)) = find_marker(&input, 14) {
        let start_of_message_marker = pos + 14;
        println!("{start_of_message_marker}")
    }
}
