use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub mod operations;
pub mod utils;

#[allow(dead_code)]
pub enum TestIdentity {
    Controller,
    User1,
    User2,
    User3,
}

#[derive(Debug)]
pub enum CanisterName {
    Index,
    Bucket,
}

impl FromStr for CanisterName {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "index" => Ok(CanisterName::Index),
            "bucket" => Ok(CanisterName::Bucket),
            _ => Err(format!("Unrecognised canister name: {s}")),
        }
    }
}

impl Display for CanisterName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            CanisterName::Index => "index",
            CanisterName::Bucket => "bucket",
        };

        f.write_str(name)
    }
}
