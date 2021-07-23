struct UserID(i32);
struct UserName(String);
struct User {
    id: UserID,
    name: UserName,
}

trait UserFactory {
    fn create(&self, name: UserName) -> User;
}

// InMemory(メモリ上でidを振る)やDBなど生成方法を抽象化
// 抽象化の必要がないならFactoryの意味がない
struct UserFactoryImpl {
    current_id: i32,
}

impl UserFactory for UserFactoryImpl {
    fn create(&self, name: UserName) -> User {
        User {
            id: UserID { 0: self.current_id },
            name,
        }
    }
}

struct UserApplicationService {
    user_factory: Box<dyn UserFactory>,
}

impl UserApplicationService {
    fn create(&self, name: UserName) -> User {
        self.user_factory.create(name)
    }
}

#[test]
fn test_factory() {
    let service = UserApplicationService {
        user_factory: Box::new(UserFactoryImpl { current_id: 0 }),
    };
    let _ = service.create(UserName {
        0: "name".to_string(),
    });
}
