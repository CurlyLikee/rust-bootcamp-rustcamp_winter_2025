use task_1_5::{EmailString, Random};


fn main() {
    let random = Random::new("test@example.com", "test@example", "test");
    let email_candidate = *random;

    match EmailString::try_from(email_candidate) {
        Ok(email) => println!("Valid email: {}", email.into_inner()),
        Err(err) => println!("Invalid email: {}", err),
    }
}
