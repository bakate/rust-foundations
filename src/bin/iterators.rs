fn select_long_topics(
    topics: &[String],
    minimum_character_count: usize,
) -> Vec<String> {
    topics
        .iter()
        .filter(|topic| {
            topic.chars().count() >= minimum_character_count
        })
        .cloned()
        .collect()
}

fn main() {
    let topics = vec![
        String::from("Ownership"),
        String::from("Error handling"),
        String::from("Traits"),
    ];

    let long_topics = select_long_topics(&topics, 10);

    println!("Original: {topics:?}");
    println!("Long topics: {long_topics:?}");
}
