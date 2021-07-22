#[derive(Debug, PartialEq, Eq)]
struct UserID(String);
impl UserID {
    fn new(value: &str) -> Self {
        Self {
            0: value.to_string(),
        }
    }
}

static mut users: Vec<User> = vec![];

struct User {
    user_id: UserID,
    name: String,
}

impl User {
    fn new(user_id: UserID, name: &str) -> &'static Self {
        let user = Self {
            user_id,
            name: name.to_string(),
        };
        unsafe {
            users.push(user);
            users.last().unwrap()
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.user_id == other.user_id
    }
}

struct UserService {}
impl UserService {
    fn exists(user: &User) -> bool {
        unsafe { users.iter().any(|u| u == user) }
    }
}

#[test]
fn check_user_exist() {
    let user_a = User::new(UserID::new("user_a_id"), "user_a_name");
    assert!(UserService::exists(user_a));
    assert!(!UserService::exists(&User {
        user_id: UserID::new("user_b_id"),
        name: "user_b_name".to_string(),
    }))
}
