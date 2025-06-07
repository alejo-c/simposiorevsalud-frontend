use regex::Regex;

pub fn validate_password(password: &str) -> bool {
    let regex = Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*[0-9])(?=.*[!@#$%^&*_=+']).*$").unwrap();
    regex.is_test(password)
}

pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.len() >= 5
}
