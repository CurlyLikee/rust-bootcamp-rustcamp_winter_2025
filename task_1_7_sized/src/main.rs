use std::borrow::Cow;
use task_1_7::{CreateUser, CreateUserHandler, InMemoryUserRepository, CommandHandler};


fn main() {
    let repo = InMemoryUserRepository::new();
    let mut handler = CreateUserHandler::new(repo);

    let command = CreateUser {
        id: 1,
        email: Cow::Borrowed("user@example.com"),
        activated: true,
    };

    handler.handle(command);

    println!("User created successfully!");
}


