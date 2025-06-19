#[cfg(target_arch = "x86_64")]
mod x64_jit;

#[cfg(target_arch = "aarch64")]
mod aarch64_jit;

pub mod jit;
