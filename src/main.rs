use rust_foundations::{ProgressError, calculate_completion_percentage};

fn main() {
    let completed_sessions: u32 = 9;
    let target_sessions: u32 = 8;

    let result = calculate_completion_percentage(completed_sessions, target_sessions);
    match result {
        Ok(percentage) => println!("Progress: {percentage}%"),
        Err(ProgressError::ZeroTarget) => {
            println!("Target sessions must be greater than zero")
        }
        Err(ProgressError::CompletedSessionsExceedTarget {
            completed_sessions,
            target_sessions,
        }) => {
            println!(
                "Completed sessions ({completed_sessions}) cannot exceed target sessions ({target_sessions})"
            )
        }
    }
}
