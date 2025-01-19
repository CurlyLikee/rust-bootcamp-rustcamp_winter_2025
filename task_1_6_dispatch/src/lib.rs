use std::borrow::Cow;
use std::collections::HashMap;


pub trait Storage<K, V> {
    fn set(&mut self, key: K, val: V);
    fn get(&self, key: &K) -> Option<&V>;
    fn remove(&mut self, key: &K) -> Option<V>;
}



#[derive(Debug, PartialEq, Eq, Clone)]
pub struct User {
    pub id: u64,
    pub email: Cow<'static, str>,
    pub activated: bool,
}



pub struct StaticUserRepository<S: Storage<u64, User>> {
    storage: S,
}


impl<S: Storage<u64, User>> StaticUserRepository<S> {
    pub fn new(storage: S) -> Self {
        StaticUserRepository { storage }
    }

    pub fn add_user(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    pub fn get_user(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    pub fn update_user(&mut self, id: u64, email: Option<Cow<'static, str>>, activated: Option<bool>) {
        if let Some(user) = self.storage.get(&id).cloned() {
            let updated_user = User {
                id: user.id,
                email: email.unwrap_or(user.email),
                activated: activated.unwrap_or(user.activated),
            };
            self.storage.set(id, updated_user);
        }
    }

    pub fn remove_user(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }
}






pub struct DynamicUserRepository {
    storage: Box<dyn Storage<u64, User>>,
}

impl DynamicUserRepository {
    pub fn new(storage: Box<dyn Storage<u64, User>>) -> Self {
        DynamicUserRepository { storage }
    }

    pub fn add_user(&mut self, user: User) {
        self.storage.set(user.id, user);
    }

    pub fn get_user(&self, id: u64) -> Option<&User> {
        self.storage.get(&id)
    }

    pub fn update_user(&mut self, id: u64, email: Option<Cow<'static, str>>, activated: Option<bool>) {
        if let Some(user) = self.storage.get(&id).cloned() {
            let updated_user = User {
                id: user.id,
                email: email.unwrap_or(user.email),
                activated: activated.unwrap_or(user.activated),
            };
            self.storage.set(id, updated_user);
        }
    }

    pub fn remove_user(&mut self, id: u64) -> Option<User> {
        self.storage.remove(&id)
    }
}





pub struct InMemoryStorage<K, V> {
    data: HashMap<K, V>,
}

impl<K, V> InMemoryStorage<K, V>
where
    K: Eq + std::hash::Hash,
{
    pub fn new() -> Self {
        InMemoryStorage {
            data: HashMap::new(),
        }
    }
}

impl<K, V> Storage<K, V> for InMemoryStorage<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn set(&mut self, key: K, val: V) {
        self.data.insert(key, val);
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_repository() {
        let mut storage = InMemoryStorage::new();
        let mut repo = StaticUserRepository::new(storage);

        let user1 = User {
            id: 1,
            email: Cow::Borrowed("user1@example.com"),
            activated: true,
        };
        repo.add_user(user1.clone());

        assert_eq!(repo.get_user(1), Some(&user1));

        repo.update_user(1, Some(Cow::Borrowed("updated@example.com")), Some(false));
        assert_eq!(
            repo.get_user(1),
            Some(&User {
                id: 1,
                email: Cow::Borrowed("updated@example.com"),
                activated: false,
            })
        );

        let removed_user = repo.remove_user(1);
        assert_eq!(removed_user, Some(User {
            id: 1,
            email: Cow::Borrowed("updated@example.com"),
            activated: false,
        }));
        assert_eq!(repo.get_user(1), None);
    }



    
    #[test]
    fn test_dynamic_repository() {
        let storage = InMemoryStorage::new();
        let mut repo = DynamicUserRepository::new(Box::new(storage));

        let user1 = User {
            id: 1,
            email: Cow::Borrowed("user1@example.com"),
            activated: true,
        };
        repo.add_user(user1.clone());

        assert_eq!(repo.get_user(1), Some(&user1));

        repo.update_user(1, Some(Cow::Borrowed("updated@example.com")), Some(false));
        assert_eq!(
            repo.get_user(1),
            Some(&User {
                id: 1,
                email: Cow::Borrowed("updated@example.com"),
                activated: false,
            })
        );

        let removed_user = repo.remove_user(1);
        assert_eq!(removed_user, Some(User {
            id: 1,
            email: Cow::Borrowed("updated@example.com"),
            activated: false,
        }));
        assert_eq!(repo.get_user(1), None);
    }
}
