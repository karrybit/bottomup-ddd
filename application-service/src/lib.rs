#[derive(Debug, Clone, PartialEq, Eq)]
struct UserID(String);
impl UserID {
    fn new(id: &str) -> Self {
        Self { 0: id.to_string() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UserName(String);
impl UserName {
    fn new(name: &str) -> Self {
        Self {
            0: name.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UserMailAddress(String);
impl UserMailAddress {
    fn new(mail_address: &str) -> Self {
        Self {
            0: mail_address.to_string(),
        }
    }
}

struct User {
    id: UserID,
    name: UserName,
    mail_address: UserMailAddress,
}

impl User {
    fn new(name: UserName, mail_address: UserMailAddress) -> Self {
        let uuid = uuid::Uuid::new_v4().to_string();
        Self {
            id: UserID::new(&uuid),
            name,
            mail_address,
        }
    }

    fn change_name(&mut self, name: UserName) {
        self.name = name;
    }

    fn change_mail_address(&mut self, mail_address: UserMailAddress) {
        self.mail_address = mail_address
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

struct UserData<'a> {
    id: &'a UserID,
    name: &'a UserName,
}

trait UserRepository {
    fn find_by_id(&self, id: UserID) -> Option<&User>;
    fn find_by_id_mut(&self, id: UserID) -> Option<&mut User>;
    fn find_by_name(&self, name: UserName) -> Option<&User>;
    fn find_by_name_mut(&self, name: UserName) -> Option<&mut User>;
    fn save(&self, user: &User);
    fn delete(&self, user: &User);
}

struct UserService {
    user_repository: Box<dyn UserRepository>,
}

impl UserService {
    fn new(user_repository: Box<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    fn exists(&self, user: &User) -> bool {
        self.user_repository.find_by_id(user.id.clone()).is_some()
    }
}

struct UserUpdateCommand {
    id: String,
    name: Option<String>,
    mail_address: Option<String>,
}

impl UserUpdateCommand {
    fn new(id: &str, name: Option<&str>, mail_address: Option<&str>) -> Self {
        Self {
            id: id.to_string(),
            name: name.map(str::to_string),
            mail_address: mail_address.map(str::to_string),
        }
    }
}

struct UserDeleteCommand {
    id: String,
}

struct UserApplicationService {
    user_repository: Box<dyn UserRepository>,
    user_service: UserService,
}

impl UserApplicationService {
    fn new(user_repository: Box<dyn UserRepository>, user_service: UserService) -> Self {
        Self {
            user_repository,
            user_service,
        }
    }
    fn register(&self, name: &str, mail_address: &str) {
        let user = User::new(UserName::new(name), UserMailAddress::new(mail_address));
        if !self.user_service.exists(&user) {
            self.user_repository.save(&user);
        }
    }

    fn get(&self, user_id: &str) -> Option<UserData> {
        let user_id = UserID::new(user_id);
        let user = self.user_repository.find_by_id(user_id)?;
        Some(UserData {
            id: &user.id,
            name: &user.name,
        })
    }

    fn update(&self, command: UserUpdateCommand) {
        let user_repository = self.user_repository.as_ref();
        let user = user_repository
            .find_by_id_mut(UserID::new(&command.id))
            .expect("user is not found");

        if let Some(name) = command.name {
            user.change_name(UserName::new(&name));
            if !self.user_service.exists(&user) {
                panic!("user is already exist");
            }
        }

        if let Some(mail_address) = command.mail_address {
            user.change_mail_address(UserMailAddress::new(&mail_address));
        }

        user_repository.save(user);
    }

    fn delete(&self, command: UserDeleteCommand) {
        let user = self
            .user_repository
            .find_by_id(UserID::new(&command.id))
            .unwrap();
        self.user_repository.delete(user);
    }
}

struct Client {
    user_application_service: UserApplicationService,
}

impl Client {
    fn change_name(&self, id: &str, name: &str) {
        self.user_application_service
            .update(UserUpdateCommand::new(id, Some(name), None));
    }

    fn change_mail_address(&self, id: &str, mail_address: &str) {
        self.user_application_service
            .update(UserUpdateCommand::new(id, None, Some(mail_address)));
    }
}
