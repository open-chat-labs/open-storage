mod hash;

use candid::Principal;

pub use hash::*;

pub type BlobId = u128;
pub type CanisterId = Principal;
pub type Cycles = u128;
pub type Milliseconds = u64;
pub type TimestampMillis = u64;
pub type TimestampNanos = u64;
