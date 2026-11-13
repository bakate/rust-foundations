struct LearningPlan {
    language: String,
    topics: Vec<String>,
}

impl LearningPlan {
    fn new(language: String) -> Self {
        Self {
            language,
            topics: Vec::new(),
        }
    }

    fn add_topic(&mut self, topic: String) {
        self.topics.push(topic);
    }

    fn topic_count(&self) -> usize {
        self.topics.len()
    }

    fn print_topics(&self) {
        for (topic_index, topic) in self.topics.iter().enumerate() {
            println!("{}. {topic}", topic_index + 1)
        }
    }

    fn mark_topics_as_planned(&mut self) {
        for topic in self.topics.iter_mut() {
            topic.push_str(" [planned]");
        }
    }

    fn drain_topics(&mut self) {
        for topic in self.topics.drain(..) {
            println!("Consumed: {topic}");
        }
    }
}

fn main() {
    let mut learning_plan = LearningPlan::new(String::from("Rust"));

    learning_plan.add_topic(String::from("Ownership"));
    learning_plan.add_topic(String::from("Error handling"));

    learning_plan.mark_topics_as_planned();
    learning_plan.print_topics();
    learning_plan.drain_topics();

    println!("Language: {}", learning_plan.language);

    println!("Remaining topics: {}", learning_plan.topic_count(),);
}
