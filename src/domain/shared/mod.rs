pub mod repositories;

pub trait UsecaseSpecification<T, R> {
    fn execute(&self, value: T) -> R;
}

pub trait Validator<T> {
    fn is_valid(&self, value: T) -> bool;
}

pub trait CompositeValidator<T>: Validator<T> {
    fn add_validator(&self, validator: Box<dyn Validator<T>>) -> Self;

    fn remove_validator(&self, validator: Box<dyn Validator<T>>) -> Self;
}

