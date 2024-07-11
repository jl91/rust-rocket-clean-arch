pub mod repositories;
mod validators;

pub trait UsecaseSpecification<T, R> {
    fn execute(&self, value: T) -> R;
}

