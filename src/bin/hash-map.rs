use std::collections::HashMap;

fn record_session(completed_session_by_topic: &mut HashMap<String, u32>, topic: &str) {
    let completed_sessions = completed_session_by_topic
        .entry(topic.to_owned())
        .or_insert(0);

    *completed_sessions += 1;
}

fn main() {
    let mut completed_sessions_by_topic = HashMap::<String, u32>::new();

    completed_sessions_by_topic.insert(String::from("Ownership"), 3);

    completed_sessions_by_topic.insert(String::from("Error handling"), 2);

    let searched_topic = "Ownership";

    match completed_sessions_by_topic.get(searched_topic) {
        Some(completed_sessions) => {
            println!("{searched_topic}: {completed_sessions} sessions");
        }
        None => {
            println!("Unknown topic: {searched_topic}");
        }
    }

    record_session(&mut completed_sessions_by_topic, "Ownership");

    record_session(&mut completed_sessions_by_topic, "Lifetimes");
}
