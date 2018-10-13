// CACHE_LINE_SIZE is the size of a cache line on this specific architecture.
#[cfg(target_arch = "x86")]
pub const CACHE_LINE_SIZE: usize = 64;
#[cfg(target_arch = "x86_64")]
pub const CACHE_LINE_SIZE: usize = 64;
#[cfg(target_arch = "mips")]
pub const CACHE_LINE_SIZE: usize = 32;
#[cfg(target_arch = "powerpc")]
pub const CACHE_LINE_SIZE: usize = 64;
#[cfg(target_arch = "powerpc64")]
pub const CACHE_LINE_SIZE: usize = 128;
#[cfg(target_arch = "arm")]
pub const CACHE_LINE_SIZE: usize = 32;
#[cfg(target_arch = "aarch64")]
pub const CACHE_LINE_SIZE: usize = 32; // aka arm64
#[cfg(target_arch = "s390x")]
pub const CACHE_LINE_SIZE: usize = 256;
