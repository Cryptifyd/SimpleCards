use crate::utils::errors::AppError;
use regex::Regex;
use std::sync::OnceLock;

// Email validation regex
static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();

fn email_regex() -> &'static Regex {
    EMAIL_REGEX.get_or_init(|| {
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
    })
}

pub fn validate_email(email: &str) -> Result<(), AppError> {
    if email.is_empty() {
        return Err(AppError::Validation("Email is required".to_string()));
    }

    if email.len() > 255 {
        return Err(AppError::Validation("Email must be 255 characters or less".to_string()));
    }

    if !email_regex().is_match(email) {
        return Err(AppError::Validation("Invalid email format".to_string()));
    }

    Ok(())
}

pub fn validate_username(username: &str) -> Result<(), AppError> {
    if username.is_empty() {
        return Err(AppError::Validation("Username is required".to_string()));
    }

    if username.len() < 3 {
        return Err(AppError::Validation("Username must be at least 3 characters".to_string()));
    }

    if username.len() > 50 {
        return Err(AppError::Validation("Username must be 50 characters or less".to_string()));
    }

    // Only allow alphanumeric characters and underscores
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(AppError::Validation(
            "Username can only contain letters, numbers, and underscores".to_string(),
        ));
    }

    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), AppError> {
    if password.is_empty() {
        return Err(AppError::Validation("Password is required".to_string()));
    }

    if password.len() < 8 {
        return Err(AppError::Validation("Password must be at least 8 characters".to_string()));
    }

    if password.len() > 128 {
        return Err(AppError::Validation("Password must be 128 characters or less".to_string()));
    }

    // Check for at least one lowercase, uppercase, digit, and special character
    let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));

    if !has_lowercase {
        return Err(AppError::Validation("Password must contain at least one lowercase letter".to_string()));
    }

    if !has_uppercase {
        return Err(AppError::Validation("Password must contain at least one uppercase letter".to_string()));
    }

    if !has_digit {
        return Err(AppError::Validation("Password must contain at least one digit".to_string()));
    }

    if !has_special {
        return Err(AppError::Validation("Password must contain at least one special character".to_string()));
    }

    Ok(())
}

pub fn validate_display_name(display_name: &str) -> Result<(), AppError> {
    if display_name.is_empty() {
        return Err(AppError::Validation("Display name is required".to_string()));
    }

    if display_name.len() > 255 {
        return Err(AppError::Validation("Display name must be 255 characters or less".to_string()));
    }

    Ok(())
}

pub fn validate_hex_color(color: &str) -> Result<(), AppError> {
    if !color.starts_with('#') || color.len() != 7 {
        return Err(AppError::Validation("Color must be a valid hex color code (e.g., #FF0000)".to_string()));
    }

    if !color[1..].chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(AppError::Validation("Color must be a valid hex color code".to_string()));
    }

    Ok(())
}

pub fn validate_team_name(name: &str) -> Result<(), AppError> {
    if name.is_empty() {
        return Err(AppError::Validation("Team name is required".to_string()));
    }

    if name.len() < 2 {
        return Err(AppError::Validation("Team name must be at least 2 characters".to_string()));
    }

    if name.len() > 100 {
        return Err(AppError::Validation("Team name must be 100 characters or less".to_string()));
    }

    Ok(())
}

pub fn validate_team_description(description: &str) -> Result<(), AppError> {
    if description.len() > 500 {
        return Err(AppError::Validation("Team description must be 500 characters or less".to_string()));
    }

    Ok(())
}

pub fn validate_project_name(name: &str) -> Result<(), AppError> {
    if name.is_empty() {
        return Err(AppError::Validation("Project name is required".to_string()));
    }

    if name.len() < 2 {
        return Err(AppError::Validation("Project name must be at least 2 characters".to_string()));
    }

    if name.len() > 100 {
        return Err(AppError::Validation("Project name must be 100 characters or less".to_string()));
    }

    Ok(())
}

pub fn validate_project_description(description: &str) -> Result<(), AppError> {
    if description.len() > 1000 {
        return Err(AppError::Validation("Project description must be 1000 characters or less".to_string()));
    }

    Ok(())
}

pub fn validate_task_title(title: &str) -> Result<(), AppError> {
    if title.is_empty() {
        return Err(AppError::Validation("Task title is required".to_string()));
    }

    if title.len() < 2 {
        return Err(AppError::Validation("Task title must be at least 2 characters".to_string()));
    }

    if title.len() > 255 {
        return Err(AppError::Validation("Task title must be 255 characters or less".to_string()));
    }

    Ok(())
}

pub fn validate_task_description(description: &str) -> Result<(), AppError> {
    if description.len() > 2000 {
        return Err(AppError::Validation("Task description must be 2000 characters or less".to_string()));
    }

    Ok(())
}

pub fn validate_board_name(name: &str) -> Result<(), AppError> {
    if name.is_empty() {
        return Err(AppError::Validation("Board name is required".to_string()));
    }

    if name.len() < 2 {
        return Err(AppError::Validation("Board name must be at least 2 characters".to_string()));
    }

    if name.len() > 100 {
        return Err(AppError::Validation("Board name must be 100 characters or less".to_string()));
    }

    Ok(())
}

pub fn validate_board_description(description: &str) -> Result<(), AppError> {
    if description.len() > 500 {
        return Err(AppError::Validation("Board description must be 500 characters or less".to_string()));
    }

    Ok(())
}

pub fn validate_task_comment(content: &str) -> Result<(), AppError> {
    if content.is_empty() {
        return Err(AppError::Validation("Comment content is required".to_string()));
    }

    if content.len() > 1000 {
        return Err(AppError::Validation("Comment must be 1000 characters or less".to_string()));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("user.name+tag@domain.co.uk").is_ok());
        
        assert!(validate_email("").is_err());
        assert!(validate_email("invalid-email").is_err());
        assert!(validate_email("@domain.com").is_err());
        assert!(validate_email("user@").is_err());
    }

    #[test]
    fn test_username_validation() {
        assert!(validate_username("user123").is_ok());
        assert!(validate_username("test_user").is_ok());
        
        assert!(validate_username("").is_err());
        assert!(validate_username("us").is_err());
        assert!(validate_username("user@domain").is_err());
        assert!(validate_username("user-name").is_err());
    }

    #[test]
    fn test_password_validation() {
        assert!(validate_password("Password123!").is_ok());
        
        assert!(validate_password("").is_err());
        assert!(validate_password("short").is_err());
        assert!(validate_password("onlylowercase123!").is_err());
        assert!(validate_password("ONLYUPPERCASE123!").is_err());
        assert!(validate_password("NoDigits!").is_err());
        assert!(validate_password("NoSpecialChars123").is_err());
    }

    #[test]
    fn test_hex_color_validation() {
        assert!(validate_hex_color("#FF0000").is_ok());
        assert!(validate_hex_color("#00ff00").is_ok());
        assert!(validate_hex_color("#123ABC").is_ok());
        
        assert!(validate_hex_color("FF0000").is_err());
        assert!(validate_hex_color("#FF00").is_err());
        assert!(validate_hex_color("#GG0000").is_err());
    }
}