use task_1_9::Fact;

fn main() {
    let vec_fact: Fact<Vec<i32>> = Fact::default();
    println!("Fact about Vec: {}", vec_fact.fact());
    println!("Fact about Vec: {}", vec_fact.fact());

    let string_fact: Fact<String> = Fact::default();
    println!("Fact about String: {}", string_fact.fact());
    println!("Fact about String: {}", string_fact.fact());

    let int_fact: Fact<i32> = Fact::default();
    println!("Fact about i32: {}", int_fact.fact());
    println!("Fact about i32: {}", int_fact.fact());
}
