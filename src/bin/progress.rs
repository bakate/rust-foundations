use rust_foundations::{ProgressError, calculate_completion_percentage};
use std::env;
use std::num::ParseIntError;
use std::process::ExitCode;

enum CliError {
    InvalidArguments,
    InvalidSessionCount(ParseIntError),
    Progress(ProgressError),
}

fn parse_session_counts(
    completed_sessions: &str,
    target_sessions: &str,
) -> Result<(u32, u32), CliError> {
    let completed_sessions = completed_sessions
        .parse::<u32>()
        .map_err(CliError::InvalidSessionCount)?;

    let target_sessions = target_sessions
        .parse::<u32>()
        .map_err(CliError::InvalidSessionCount)?;

    Ok((completed_sessions, target_sessions))
}

fn run(arguments: &[String]) -> Result<u32, CliError> {
    let [_, completed_sessions, target_sessions] = arguments else {
        return Err(CliError::InvalidArguments);
    };

    let (completed_sessions, target_sessions) =
        parse_session_counts(completed_sessions, target_sessions)?;

    calculate_completion_percentage(completed_sessions, target_sessions).map_err(CliError::Progress)
}

fn print_error(error: CliError) {
    match error {
        CliError::InvalidArguments => {
            println!("Usage: progress <completed-sessions> <target-sessions>");
        }
        CliError::InvalidSessionCount(error) => {
            println!("Invalid session count: {error}");
        }
        CliError::Progress(ProgressError::ZeroTarget) => {
            println!("Target sessions must be greater than zero");
        }
        CliError::Progress(ProgressError::CompletedSessionsExceedTarget {
            completed_sessions,
            target_sessions,
        }) => {
            println!(
                "Completed sessions ({completed_sessions}) cannot exceed target sessions ({target_sessions})"
            );
        }
    }
}

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().collect();

    match run(&arguments) {
        Ok(percentage) => {
            println!("Progress: {percentage}%");
            ExitCode::SUCCESS
        }
        Err(error) => {
            print_error(error);
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_percentage_from_valid_arguments() {
        let arguments = vec![
            String::from("progress"),
            String::from("5"),
            String::from("8"),
        ];

        let result = run(&arguments);
        assert!(matches!(result, Ok(62)))
    }

    #[test]
    fn returns_invalid_arguments_error() {
        let arguments = vec![String::from("progress")];

        let result = run(&arguments);
        assert!(matches!(result, Err(CliError::InvalidArguments)))
    }

    #[test]
    fn returns_invalid_session_count_error() {
        let arguments = vec![
            String::from("progress"),
            String::from("invalid"),
            String::from("8"),
        ];

        let result = run(&arguments);

        assert!(matches!(result, Err(CliError::InvalidSessionCount(_))));
    }

    #[test]
    fn returns_zero_target_progress_error() {
        let arguments = vec![
            String::from("progress"),
            String::from("5"),
            String::from("0"),
        ];

        let result = run(&arguments);

        assert!(matches!(
            result,
            Err(CliError::Progress(ProgressError::ZeroTarget))
        ));
    }

    #[test]
    fn returns_completed_sessions_exceed_target_progress_error() {
        let arguments = vec![
            String::from("progress"),
            String::from("9"),
            String::from("8"),
        ];

        let result = run(&arguments);

        assert!(matches!(
            result,
            Err(CliError::Progress(
                ProgressError::CompletedSessionsExceedTarget {
                    completed_sessions: 9,
                    target_sessions: 8,
                }
            ))
        ));
    }
}
