#![feature(asm)]
#![feature(cfg_target_feature)]

mod generic;
#[cfg(target_arch="x86_64")]mod x86_64;

pub use generic::exp::exp;

#[cfg(all(target_arch="x86_64", target_feature="sse"))]
pub use x86_64::sse::sqrt::sqrt;
#[cfg(all(target_arch="x86_64", not(target_feature="sse")))]
pub use x86_64::sqrt::sqrt;
#[cfg(not(target_arch="x86_64"))]
pub use generic::sqrt::sqrt;
