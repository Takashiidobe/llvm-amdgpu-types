#![doc = include_str!("../README.md")]

mod decode;
mod generated;
mod instruction;
mod operand;

pub use decode::DecodeError;
pub use generated::*;
pub use instruction::*;
pub use operand::*;

mod private {
    pub trait Sealed {}
    pub trait Compare<P, T, E> {}
    pub trait Unary<F, D, S, E> {}
    pub trait Binary<F, D, S0, S1, E> {}
    pub trait Ternary<F, D, S0, S1, S2, E> {}
}
