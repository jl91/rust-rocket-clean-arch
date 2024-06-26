
pub mod repositories;

pub trait UsecaseSpecification<T, R> {
    fn execute(&self, value: T) -> R;
}