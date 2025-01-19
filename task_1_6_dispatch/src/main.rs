use task_1_6::{DynamicUserRepository, InMemoryStorage, StaticUserRepository, User};
use std::borrow::Cow;


fn main() {
    let mut static_storage = InMemoryStorage::new();
    let mut static_repo = StaticUserRepository::new(static_storage);

    let user1 = User {
        id: 1,
        email: Cow::Borrowed("user1@example.com"),
        activated: true,
    };
    static_repo.add_user(user1.clone());
    println!("Static Repo - User 1: {:?}", static_repo.get_user(1));

    static_repo.update_user(1, Some(Cow::Borrowed("updated@example.com")), None);
    println!("Static Repo - Updated User 1: {:?}", static_repo.get_user(1));

    static_repo.remove_user(1);
    println!("Static Repo - User 1 after removal: {:?}", static_repo.get_user(1));

    let dynamic_storage = InMemoryStorage::new();
    let mut dynamic_repo = DynamicUserRepository::new(Box::new(dynamic_storage));

    let user2 = User {
        id: 2,
        email: Cow::Borrowed("user2@example.com"),
        activated: false,
    };
    dynamic_repo.add_user(user2.clone());
    println!("Dynamic Repo - User 2: {:?}", dynamic_repo.get_user(2));

    dynamic_repo.update_user(2, Some(Cow::Borrowed("dynamic@example.com")), Some(true));
    println!("Dynamic Repo - Updated User 2: {:?}", dynamic_repo.get_user(2));

    dynamic_repo.remove_user(2);
    println!("Dynamic Repo - User 2 after removal: {:?}", dynamic_repo.get_user(2));
}
