use regex::Regex;

pub fn validate_password(password: &str) -> bool {
    // Check minimum length
    if password.len() < 8 {
        return false;
    }

    // Check for at least one lowercase letter
    let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());

    // Check for at least one uppercase letter
    let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());

    // Check for at least one digit
    let has_digit = password.chars().any(|c| c.is_ascii_digit());

    // Check for at least one special character
    let special_chars = "!@#$%^&*_=+'";
    let has_special = password.chars().any(|c| special_chars.contains(c));

    has_lowercase && has_uppercase && has_digit && has_special
}

pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && email.len() >= 5
}
