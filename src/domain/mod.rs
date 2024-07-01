use std::fmt::{Formatter, write};

pub mod usecases;
pub mod entities;
pub mod shared;

struct DomainError;
// impl std::fmt::Display for  DomainError{
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write(f, "DomainError")
//     }
// }
