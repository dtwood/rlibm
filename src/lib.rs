#![feature(asm)]
#![feature(cfg_target_feature)]

#[cfg(target_arch="x86_64")] mod x86_64;
mod generic;

pub use generic::exp::exp;
#[cfg(all(target_arch="x86_64", target_feature="sse"))] pub use x86_64::sse::sqrt::sqrt;
