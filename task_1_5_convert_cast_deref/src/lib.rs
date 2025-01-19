use rand::Rng;
use std::convert::TryFrom;



pub struct EmailString(String);

impl EmailString {
    pub fn try_from(value: &str) -> Result<Self, String> {
        if value.contains('@') && value.contains('.') {
            Ok(Self(value.to_string()))
        } else {
            Err(format!("Invalid email: {}", value))
        }
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl AsRef<str> for EmailString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}




pub struct Random<T> {
    values: [T; 3],
}

impl<T: Clone> Random<T> {
    pub fn new(a: T, b: T, c: T) -> Self {
        Self {
            values: [a, b, c],
        }
    }
}

impl<T: Clone> std::ops::Deref for Random<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        let idx = rand::thread_rng().gen_range(0..3);
        &self.values[idx]
    }
}

impl<T: Clone> TryFrom<[T; 3]> for Random<T> {
    type Error = String;

    fn try_from(values: [T; 3]) -> Result<Self, Self::Error> {
        Ok(Self { values })
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_string() {
        assert!(EmailString::try_from("test@example.com").is_ok());
        assert!(EmailString::try_from("invalid-email").is_err());
    }

    #[test]
    fn test_random() {
        let random = Random::new(1, 2, 3);
        let value = *random;
        assert!(value == 1 || value == 2 || value == 3);
    }
}