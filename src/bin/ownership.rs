struct LearningPlan {
    title: String,
}

impl LearningPlan {
    fn new(title: String) -> Self {
        Self { title }
    }

    fn character_count(&self) -> usize {
        self.title.chars().count()
    }

    fn append_topic(&mut self, topic: &str) {
        self.title.push_str(" -> ");
        self.title.push_str(topic);
    }
}

fn main() {
    let mut learning_plan = LearningPlan::new(String::from("Rust"));

    learning_plan.append_topic("Ownership");

    println!(
        "Plan: {} ({} characters)",
        learning_plan.title,
        learning_plan.character_count(),
    );
}
