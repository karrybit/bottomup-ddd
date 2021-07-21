struct UserName(String);
impl UserName {
    fn new(value: String) -> Self {
        if value.len() < 3 {
            panic!("ユーザ名は3文字以上です。")
        }
        Self { 0: value }
    }
}

struct UserID(String);
impl UserID {
    fn new(value: String) -> Self {
        if value.is_empty() {
            panic!("value is empty")
        }
        Self { 0: value }
    }
}

struct User {
    id: UserID,
    name: UserName,
}
