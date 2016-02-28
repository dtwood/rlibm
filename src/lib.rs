#![feature(asm)]

mod amd64;
mod generic;

pub use generic::exp::exp;
pub use amd64::sqrt::sqrt;
