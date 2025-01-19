use rand::seq::SliceRandom;


pub struct Fact<T> {
    facts: Vec<&'static str>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Fact<T> {
    pub fn new() -> Self {
        Self {
            facts: vec![],
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_facts(facts: Vec<&'static str>) -> Self {
        Self {
            facts,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn fact(&self) -> &'static str {
        self.facts
            .choose(&mut rand::thread_rng())
            .unwrap_or(&"No facts available.")
    }
}

impl<T> Default for Fact<Vec<T>> {
    fn default() -> Self {
        Fact::with_facts(vec![
            "Vec is heap-allocated.",
            "Vec may re-allocate on growing.",
            "Vec is a growable array type in Rust.",
        ])
    }
}

impl Default for Fact<String> {
    fn default() -> Self {
        Fact::with_facts(vec![
            "String is a heap-allocated string type in Rust.",
            "String supports UTF-8 encoding.",
            "String can grow dynamically.",
        ])
    }
}

impl Default for Fact<i32> {
    fn default() -> Self {
        Fact::with_facts(vec![
            "i32 is a 32-bit signed integer type.",
            "i32 is commonly used for integers in Rust.",
            "i32 supports arithmetic operations.",
        ])
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fact_vec() {
        let vec_fact: Fact<Vec<i32>> = Fact::default();
        let possible_facts = vec![
            "Vec is heap-allocated.",
            "Vec may re-allocate on growing.",
            "Vec is a growable array type in Rust.",
        ];
        let fact = vec_fact.fact();
        assert!(
            possible_facts.contains(&fact),
            "Unexpected fact for Vec: {}",
            fact
        );
    }

    #[test]
    fn test_fact_string() {
        let string_fact: Fact<String> = Fact::default();
        let possible_facts = vec![
            "String is a heap-allocated string type in Rust.",
            "String supports UTF-8 encoding.",
            "String can grow dynamically.",
        ];
        let fact = string_fact.fact();
        assert!(
            possible_facts.contains(&fact),
            "Unexpected fact for String: {}",
            fact
        );
    }

    #[test]
    fn test_fact_i32() {
        let int_fact: Fact<i32> = Fact::default();
        let possible_facts = vec![
            "i32 is a 32-bit signed integer type.",
            "i32 is commonly used for integers in Rust.",
            "i32 supports arithmetic operations.",
        ];
        let fact = int_fact.fact();
        assert!(
            possible_facts.contains(&fact),
            "Unexpected fact for i32: {}",
            fact
        );
    }

    #[test]
    fn test_fact_with_custom_facts() {
        let custom_facts = vec!["Custom fact 1", "Custom fact 2", "Custom fact 3"];
        let custom_fact: Fact<i64> = Fact::with_facts(custom_facts.clone());
        let fact = custom_fact.fact();
        assert!(
            custom_facts.contains(&fact),
            "Unexpected fact for custom facts: {}",
            fact
        );
    }

    #[test]
    fn test_empty_fact() {
        let empty_fact: Fact<u32> = Fact::new();
        let fact = empty_fact.fact();
        assert_eq!(fact, "No facts available.", "Empty Fact should return a default message.");
    }
}
