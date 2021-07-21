use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FirstName(String);
impl FromStr for FirstName {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err("1文字以上である必要があります".to_string())
        } else {
            Ok(FirstName(s.to_string()))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct LastName(String);
impl FromStr for LastName {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err("1文字以上である必要があります".to_string())
        } else {
            Ok(LastName(s.to_string()))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FullName {
    first_name: FirstName,
    last_name: LastName,
}

impl FullName {
    pub fn new(first_name: FirstName, last_name: LastName) -> FullName {
        FullName {
            first_name,
            last_name,
        }
    }

    pub fn first_name(&self) -> &FirstName {
        &self.first_name
    }

    pub fn last_name(&self) -> &LastName {
        &self.last_name
    }
}

#[test]
fn test_equality_of_full_name() {
    let name_a = FullName::new(
        "masanobu".parse::<FirstName>().unwrap(),
        "naruse".parse::<LastName>().unwrap(),
    );
    let name_b = FullName::new(
        "john".parse::<FirstName>().unwrap(),
        "smith".parse::<LastName>().unwrap(),
    );
    let name_c = FullName::new(
        "masanobu".parse::<FirstName>().unwrap(),
        "naruse".parse::<LastName>().unwrap(),
    );

    assert_ne!(name_a, name_b);
    assert_eq!(name_a, name_c);
}
