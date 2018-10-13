// NOTE: Any change to CACHE_LINE_SIZE should also be made to CacheAligned<T>

// CACHE_LINE_SIZE is the size of a cache line on this specific architecture.
#[cfg(target_arch = "x86")]
pub const CACHE_LINE_SIZE: usize = 64;
#[cfg(target_arch = "x86_64")]
pub const CACHE_LINE_SIZE: usize = 64;
#[cfg(target_arch = "mips")]
pub const CACHE_LINE_SIZE: usize = 32;
#[cfg(target_arch = "powerpc")]
pub const CACHE_LINE_SIZE: usize = 64; // maybe 128?
#[cfg(target_arch = "powerpc64")]
pub const CACHE_LINE_SIZE: usize = 128;
#[cfg(target_arch = "arm")]
pub const CACHE_LINE_SIZE: usize = 32;
#[cfg(target_arch = "aarch64")]
pub const CACHE_LINE_SIZE: usize = 32; // aka arm64
#[cfg(target_arch = "s390x")]
pub const CACHE_LINE_SIZE: usize = 256;

// Use CacheAligned to align a struct to the size of this architectures cache line size.
#[cfg_attr(any(target_arch = "x86_64", target_arch = "x86", target_arch = "powerpc"), repr(align(64)))]
#[cfg_attr(any(target_arch = "mips", target_arch = "arm", target_arch = "aarch64"), repr(align(32)))]
#[cfg_attr(any(target_arch = "powerpc64"), repr(align(128)))]
#[cfg_attr(any(target_arch = "s390x"), repr(align(256)))]
pub struct CacheAligned<T: Sized>(pub T);

