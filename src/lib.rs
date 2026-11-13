pub enum ProgressError {
    ZeroTarget,
    CompletedSessionsExceedTarget {
        completed_sessions: u32,
        target_sessions: u32,
    },
}

pub fn calculate_completion_percentage(
    completed_sessions: u32,
    target_sessions: u32,
) -> Result<u32, ProgressError> {
    if target_sessions == 0 {
        return Err(ProgressError::ZeroTarget);
    }

    if completed_sessions > target_sessions {
        return Err(ProgressError::CompletedSessionsExceedTarget {
            completed_sessions,
            target_sessions,
        });
    }

    Ok(completed_sessions * 100 / target_sessions)
}

// no external framework needed
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_percentage_when_target_is_valid() {
        let result = calculate_completion_percentage(5, 8);

        assert!(matches!(result, Ok(62)));
    }

    #[test]
    fn returns_zero_target_error() {
        let result = calculate_completion_percentage(5, 0);

        assert!(matches!(result, Err(ProgressError::ZeroTarget)));
    }

    #[test]
    fn returns_completed_sessions_exceed_target_error() {
        let result = calculate_completion_percentage(9, 8);

        assert!(matches!(
            result,
            Err(ProgressError::CompletedSessionsExceedTarget {
                completed_sessions: 9,
                target_sessions: 8,
            })
        ));
    }
}
