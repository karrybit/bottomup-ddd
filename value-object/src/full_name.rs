use anyhow::bail;
use regex::Regex;
use std::str::FromStr;

use my_error::MyError;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Name(String);
impl FromStr for Name {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r#"^[a-zA-Z]+$"#).unwrap();
        if regex.is_match(s) {
            Ok(Name(s.to_string()))
        } else {
            bail!(MyError::type_error("許可されていない文字が使われています"))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct FullName {
    first_name: Name,
    last_name: Name,
}

impl FullName {
    fn new(first_name: Name, last_name: Name) -> FullName {
        FullName {
            first_name,
            last_name,
        }
    }

    fn first_name(&self) -> &Name {
        &self.first_name
    }

    fn last_name(&self) -> &Name {
        &self.last_name
    }
}

#[test]
fn test_equality_of_full_name() {
    assert!("".parse::<Name>().is_err());

    let name_a = FullName::new(
        "masanobu".parse::<Name>().unwrap(),
        "naruse".parse::<Name>().unwrap(),
    );
    let name_b = FullName::new(
        "john".parse::<Name>().unwrap(),
        "smith".parse::<Name>().unwrap(),
    );
    let name_c = FullName::new(
        "masanobu".parse::<Name>().unwrap(),
        "naruse".parse::<Name>().unwrap(),
    );

    assert_ne!(name_a, name_b);
    assert_eq!(name_a, name_c);
}
