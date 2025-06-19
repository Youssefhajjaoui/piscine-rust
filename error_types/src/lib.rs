#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

pub type Utc = chrono::Utc;

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        let now = Utc::now();
        let date_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
        Self {
            form_values: (field_name, field_value),
            date: date_str,
            err: err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            let res = FormError::new("name", "".to_string(), "Username is empty");
            return Err(res);
        } else if self.password.len() < 8 {
            let res = FormError::new(
                "password",
                self.password.to_string(),
                "Password should be at least 8 characters long",
            );
            return Err(res);
        } else if !valid_password(&self.password) {
            let res = FormError::new(
                "password",
                self.password.to_string(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            );
            return Err(res);
        } else {
            Ok(())
        }
    }
}

fn valid_password(pass: &str) -> bool {
    let mut has_letter = false;
    let mut has_number = false;
    let mut has_symbol = false;

    for ch in pass.chars() {
        if ch.is_alphabetic() {
            has_letter = true;
        } else if ch.is_numeric() {
            has_number = true;
        } else if ch.is_ascii_punctuation() {
            has_symbol = true;
        }
    }

    has_letter && has_number && has_symbol
}
