pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    let mut result = String::from("");
    match security_level {
        Security::Unknown => match server {
            Ok(val) => return val.to_string(),
            Err(_) => panic!("{}", server.unwrap().to_string()),
        },
        Security::Message => match server {
            Ok(val) => return val.to_string(),
            Err(_) => panic!("{}", "ERROR: program stops".to_string()),
        },
        Security::Warning => match server {
            Ok(val) => return val.to_string(),
            Err(_) => return "WARNING: check the server".to_string(),
        },
        Security::NotFound => match server {
            Ok(val) => return val.to_string(),
            Err(val) => return format!("Not found: {}", val.to_string()),
        },
        Security::UnexpectedUrl => match server {
            Ok(val) => panic!("{}", val.to_string()),
            Err(val) => return val.to_string(),
        },
    }
}
