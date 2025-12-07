use std::fmt::Debug;

pub trait Invariant {
    const MESSAGE: &str;
}

#[diagnostic::on_unimplemented(
    message = "The invariant {Self} is not proven",
    note = "use `invariant_established!({Self}[{N}])` macro where the invariant is established."
)]
pub trait InvariantProof<const N: usize> {}

#[macro_export]
macro_rules! invariant_established {
    ($name:path [$n:literal], why = $lit:literal) => {
        #[allow(non_local_definitions)]
        impl $crate::InvariantProof<$n> for $name {}
    };
    ($name:path, why = $lit:literal) => {
        #[allow(non_local_definitions)]
        impl $crate::InvariantProof<0> for $name {}
    };
}

pub trait OptionExt<T> {
    fn unwrap_under_invariant<I: Invariant>(self) -> T;
}

impl<T> OptionExt<T> for Option<T> {
    fn unwrap_under_invariant<I: Invariant>(self) -> T {
        self.unwrap_or_else(|| {
            panic!(
                "unwrapping called on None value; violation of invariant: {}",
                I::MESSAGE
            )
        })
    }
}

pub trait ResultExt<T, E> {
    fn unwrap_under_invariant<I: Invariant>(self) -> T;
}

impl<T, E> ResultExt<T, E> for Result<T, E>
where
    E: Debug,
{
    fn unwrap_under_invariant<I: Invariant>(self) -> T {
        self.unwrap_or_else(|error| {
            panic!(
                "unwrapping called on Err value: {error:?}; violation of invariant: {}",
                I::MESSAGE
            )
        })
    }
}
