#[derive(Debug, PartialEq, Eq)]
struct UserID(String);
impl UserID {
    fn new(value: &str) -> Self {
        if value.is_empty() {
            panic!("value is empty");
        }
        Self {
            0: value.to_string(),
        }
    }
}

#[derive(Debug)]
struct User {
    user_id: UserID,
    name: String,
}

impl User {
    fn new(user_id: UserID, name: &str) -> Self {
        Self {
            user_id,
            name: name.to_string(),
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}

#[test]
fn test_equality_user() {
    let user_a = User::new(UserID::new("a"), "name_a");
    let user_b = User::new(UserID::new("a"), "name_b");
    assert_eq!(user_a, user_b);
}
