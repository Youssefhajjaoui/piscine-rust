use std::{collections::HashMap, fmt::format, num::ParseFloatError};

#[derive(Eq, Hash, PartialEq)]
pub struct Flag {
    short_hand: String,
    long_hand: String,
    desc: String,
}

impl<'a> Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        Self {
            short_hand: format!("-{}", name.chars().next().unwrap()),
            long_hand: format!("--{}", name),
            desc: d.to_string(),
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<(String, String), Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert((flag.long_hand, flag.short_hand), func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        for (key, val) in &self.flags {
            if key.0 == input || key.1 == input {
                return val(argv[0], argv[1]).map_err(|e| e.to_string());
            }
        }
        Err("invalid float literal".to_string())
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let newa: f64 = a.parse()?;
    let newb: f64 = b.parse()?;
    Ok((newa / newb).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let newa: f64 = a.parse()?;
    let newb: f64 = b.parse()?;
    Ok((newa % newb).to_string())
}
