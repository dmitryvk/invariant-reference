use invariant_reference::{OptionExt, ResultExt, invariant_established};
use invariant_reference_derive::Invariant;

fn main() {
    _ = Name::default();
    let s = User::new(123, Name::new("user".to_string()).unwrap());
    let age: u32 = s.age.try_into().unwrap_under_invariant::<AgeIsPositive>();
    let first_letter = s
        .name
        .0
        .chars()
        .next()
        .unwrap_under_invariant::<NameIsNotEmpty>();
    println!("age={age} first_letter={first_letter}");
    _ = None::<char>.unwrap_under_invariant::<NameIsNotEmpty>();
}

struct User {
    age: i32,
    name: Name,
}

#[derive(Invariant)]
struct AgeIsPositive;

impl User {
    fn new(age: i32, name: Name) -> Self {
        assert!(age > 0);
        invariant_established!(AgeIsPositive);
        Self { age, name }
    }
}

struct Name(String);
#[derive(Invariant)]
#[invariant(message = "the name is not empty", num_proofs = 2)]
struct NameIsNotEmpty;

impl Name {
    fn default() -> Self {
        invariant_established!(NameIsNotEmpty[0]);
        Self("default".into())
    }

    fn new(name: String) -> Option<Self> {
        if name.is_empty() {
            invariant_established!(NameIsNotEmpty[1]);
            return None;
        }
        Some(Self(name))
    }
}
