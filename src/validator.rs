use hunspell_rs::{CheckResult, Hunspell};
use inquire::CustomUserError;
use inquire::validator::ErrorMessage::Custom;
use inquire::validator::Validation;
use once_cell::unsync::Lazy;
use regex::Regex;
#[doc = "List of valid commit types"]
pub const VALID_TYPES: [&str; 10] = [
    "feat", "fix", "docs", "style", "refactor", "perf", "test", "chore", "ci", "build",
];
thread_local! {
    static HUNSPELL: Lazy<Hunspell> = Lazy::new(|| {
        Hunspell::new("dict/en_US.dic", "dict/en_US.aff")
    });
}

/// Validate that the input is not empty
/// # Errors
/// on bad input
pub fn validate_not_empty(input: &str) -> Result<Validation, CustomUserError> {
    if input.trim().is_empty() {
        let message = Custom(String::from("Input cannot be empty"));
        Ok(Validation::Invalid(message))
    } else {
        Ok(Validation::Valid)
    }
}
/// Validate that the input is a valid email
/// # Errors
/// on bad input
pub fn validate_email(input: &str) -> Result<Validation, CustomUserError> {
    if Regex::new(r"^[a-zA-Z0-9._+-]+@([a-zA-Z0-9-]+\.)+[a-zA-Z]{2,}$")?.is_match(input) {
        Ok(Validation::Valid)
    } else {
        let message = Custom(String::from("Invalid email format"));
        Ok(Validation::Invalid(message))
    }
}
/// Validate that the input is a valid password
/// # Errors
/// on bad input
pub fn validate_password(input: &str) -> Result<Validation, CustomUserError> {
    if input.len() >= 8 {
        Ok(Validation::Valid)
    } else {
        let message = Custom(String::from("Password must be at least 8 characters long"));
        Ok(Validation::Invalid(message))
    }
}

/// Validate that the input is a valid commit type
/// # Errors
/// on bad input
pub fn validate_commit_type(input: &str) -> Result<Validation, CustomUserError> {
    // La liste des types autorisés (basée sur Conventional Commits)

    let trimmed_input = input.trim();

    if VALID_TYPES.contains(&trimmed_input) {
        Ok(Validation::Valid)
    } else {
        let types_str = VALID_TYPES.join(", ");

        let message = Custom(format!(
            "Type '{trimmed_input}' invalide. Must be one of: {types_str}"
        ));
        Ok(Validation::Invalid(message))
    }
}
///
/// # Validate that the input is a valid spelling
///
/// # Errors
/// on bad input
///
pub fn validate_spelling(input: &str) -> Result<Validation, CustomUserError> {
    let words = input.split_whitespace();

    for word in words {
        let clean_word: String = word.chars().filter(|c| c.is_alphabetic()).collect();

        if clean_word.is_empty() {
            continue;
        }
        let is_missing =
            HUNSPELL.with(|h| h.check(&clean_word).eq(&CheckResult::MissingInDictionary));

        if is_missing {
            let suggestions = HUNSPELL.with(|h| h.suggest(&clean_word));
            let suggestions_str = suggestions.join(", ");

            let message =
                format!("Spelling error: '{clean_word}'. Suggestions: [{suggestions_str}]");
            return Ok(Validation::Invalid(Custom(message)));
        }
    }
    Ok(Validation::Valid)
}
/// Validate that the summary is under 50 characters
/// # Errors
/// on bad input
pub fn validate_summary_length(input: &str) -> Result<Validation, CustomUserError> {
    const MAX_LENGTH: usize = 50;
    let len = input.trim().len();

    if len > MAX_LENGTH {
        let message = Custom(format!(
            "Summary is too long : {len} chars. The lim is {MAX_LENGTH}."
        ));
        return Ok(Validation::Invalid(message));
    }
    Ok(Validation::Valid)
}
/// Validate that the summary does not end with a period
/// # Errors
/// on bad input
pub fn validate_summary_punctuation(input: &str) -> Result<Validation, CustomUserError> {
    if input.trim().ends_with('.') {
        let message = Custom(String::from("Summary should not end with a period."));
        Ok(Validation::Invalid(message))
    } else {
        Ok(Validation::Valid)
    }
}

/// Validate that each line of the body is under 72 characters
/// # Errors
/// on bad input
pub fn validate_body_line_length(input: &str) -> Result<Validation, CustomUserError> {
    const MAX_LINE_LENGTH: usize = 72;

    for line in input.lines() {
        if line.len() > MAX_LINE_LENGTH {
            let truncated_line = line.chars().take(20).collect::<String>();
            let message = Custom(format!(
                "The line \"{truncated_line}...\" is too long ({} chars). Limit: {MAX_LINE_LENGTH}.",
                line.len()
            ));
            return Ok(Validation::Invalid(message));
        }
    }
    Ok(Validation::Valid)
}
