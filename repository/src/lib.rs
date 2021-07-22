trait UserRepository {
    fn save(&mut self, user: User);
    fn find(&self, user_name: &UserName) -> bool;
}

#[derive(Debug, PartialEq, Eq)]
struct UserID(String);
impl UserID {
    fn new(value: &str) -> Self {
        Self {
            0: value.to_string(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct UserName(String);
impl UserName {
    fn new(value: &str) -> Self {
        Self {
            0: value.to_string(),
        }
    }
}
struct User {
    id: UserID,
    name: UserName,
}

impl User {
    fn new(id: UserID, name: UserName) -> Self {
        Self { id, name }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

struct UserService {
    user_repository: Box<dyn UserRepository>,
}

impl UserService {
    fn new(user_repository: Box<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    fn exist(&self, user_name: &UserName) -> bool {
        self.user_repository.find(user_name)
    }

    fn save(&mut self, user: User) {
        self.user_repository.save(user);
    }
}

struct PostgreSql {
    users: Vec<User>,
}
impl UserRepository for PostgreSql {
    fn save(&mut self, user: User) {
        self.users.push(user)
    }
    fn find(&self, user_name: &UserName) -> bool {
        self.users.iter().any(|user| user.name == *user_name)
    }
}

#[test]
fn test_user_repository() {
    let user = User::new(UserID::new("user_id"), UserName::new("user_name"));
    let user_name = user.name.clone();
    let mut user_service = UserService::new(Box::new(PostgreSql { users: vec![] }));
    user_service.save(user);
    assert!(user_service.exist(&user_name));
}
