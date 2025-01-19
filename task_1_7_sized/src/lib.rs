use std::borrow::Cow;
use std::collections::HashMap;



#[derive(Debug, PartialEq, Eq, Clone)]
pub struct User {
    pub id: u64,
    pub email: Cow<'static, str>,
    pub activated: bool,
}



pub struct CreateUser {
    pub id: u64,
    pub email: Cow<'static, str>,
    pub activated: bool,
}


pub trait CommandHandler<C> {
    fn handle(&mut self, command: C);
}


pub trait UserRepository {
    fn add_user(&mut self, user: User);
    fn get_user(&self, id: u64) -> Option<&User>;
    fn update_user(
        &mut self,
        id: u64,
        email: Option<Cow<'static, str>>,
        activated: Option<bool>,
    );
    fn remove_user(&mut self, id: u64) -> Option<User>;
}


pub struct InMemoryUserRepository {
    storage: HashMap<u64, User>,
}





impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }
}

impl UserRepository for InMemoryUserRepository {
    fn add_user(&mut self, user: User) {
        self.storage.insert(user.id, user);
    }

    fn get_user(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    fn update_user(
        &mut self,
        id: u64,
        email: Option<Cow<'static, str>>,
        activated: Option<bool>,
    ) {
        if let Some(user) = self.storage.get_mut(&id) {
            if let Some(new_email) = email {
                user.email = new_email;
            }
            if let Some(new_activated) = activated {
                user.activated = new_activated;
            }
        }
    }

    fn remove_user(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }
}





pub struct CreateUserHandler<R: UserRepository> {
    repository: R,
}

impl<R: UserRepository> CreateUserHandler<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

impl<R: UserRepository> CommandHandler<CreateUser> for CreateUserHandler<R> {
    fn handle(&mut self, command: CreateUser) {
        let user = User {
            id: command.id,
            email: command.email,
            activated: command.activated,
        };
        self.repository.add_user(user);
    }
}





#[cfg(test)]
mod tests {
    use super::*;
    struct MockUserRepository {
        users: HashMap<u64, User>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self {
                users: HashMap::new(),
            }
        }
    }

    impl UserRepository for MockUserRepository {
        fn add_user(&mut self, user: User) {
            self.users.insert(user.id, user);
        }

        fn get_user(&self, id: u64) -> Option<&User> {
            self.users.get(&id)
        }

        fn update_user(
            &mut self,
            id: u64,
            email: Option<Cow<'static, str>>,
            activated: Option<bool>,
        ) {
            if let Some(user) = self.users.get_mut(&id) {
                if let Some(new_email) = email {
                    user.email = new_email;
                }
                if let Some(new_activated) = activated {
                    user.activated = new_activated;
                }
            }
        }

        fn remove_user(&mut self, id: u64) -> Option<User> {
            self.users.remove(&id)
        }
    }




    #[test]
    fn test_create_user_handler() {
        let mut mock_repo = MockUserRepository::new();
        let mut handler = CreateUserHandler::new(mock_repo);

        let command = CreateUser {
            id: 1,
            email: Cow::Borrowed("test@example.com"),
            activated: true,
        };

        handler.handle(command);
        let added_user = handler.repository.get_user(1);
        assert_eq!(
            added_user,
            Some(&User {
                id: 1,
                email: Cow::Borrowed("test@example.com"),
                activated: true,
            })
        );
    }
}



