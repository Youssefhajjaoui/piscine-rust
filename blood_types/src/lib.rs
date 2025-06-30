use std::char;
use std::cmp::{Ord, Ordering};
use std::fmt::{self, Debug};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl FromStr for Antigen {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("{s}");
        match s {
            "A" => Ok(Antigen::A),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            "AB" => Ok(Antigen::AB),
            _ => Err("No matches".to_string()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("{s}");
        match s {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            _ => Err("no matches".to_string()),
        }
    }
}

impl FromStr for BloodType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 3 || s.len() < 2 {
            return Err("no matches".to_string());
        }
        // let (rh, ant) = (s[..0], s[0..]);
        let mut rh = String::from("");
        rh = s[s.len()-1..].to_string();
        let mut ant = String::from("");
        ant = s[..s.len()-1].to_string();

        // println!("{rh}             {ant}");
        Ok(BloodType {
            antigen: Antigen::from_str(&ant).expect("no matches"),
            rh_factor: RhFactor::from_str(&rh).expect("no matches"),
        })
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => self.rh_factor.cmp(&other.rh_factor),
            non_eq => non_eq,
        }
    }
}

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.antigen);
        if self.rh_factor == RhFactor::Negative {
            write!(f, "-");
        } else {
            write!(f, "+");
        }
        Ok(())
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        if self.rh_factor == RhFactor::Negative && self.rh_factor != other.rh_factor {
            return false;
        }
        if other.antigen == Antigen::O {
            return true;
        }

        self.antigen == Antigen::AB || other.antigen == self.antigen
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut res: Vec<Self> = vec![];
        for typ in [Antigen::AB, Antigen::O, Antigen::A, Antigen::B] {
            for rh in [RhFactor::Negative, RhFactor::Positive] {
                let other = BloodType {
                    rh_factor: rh,
                    antigen: typ.clone(),
                };
                if self.can_receive_from(&other) {
                    res.push(other);
                }
            }
        }
        res
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        let mut res: Vec<Self> = vec![];
        for typ in [Antigen::AB, Antigen::O, Antigen::A, Antigen::B] {
            for rh in [RhFactor::Negative, RhFactor::Positive] {
                let other = BloodType {
                    rh_factor: rh,
                    antigen: typ.clone(),
                };
                if other.can_receive_from(&self) {
                    res.push(other);
                }
            }
        }
        res
    }
}
