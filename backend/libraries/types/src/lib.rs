use candid::Principal;

mod timestamped;
mod version;

pub use timestamped::*;
pub use version::*;

pub type AccessorId = Principal;
pub type BlobId = u128;
pub type CanisterId = Principal;
pub type Cycles = u128;
pub type Hash = u128;
pub type Milliseconds = u64;
pub type TimestampMillis = u64;
pub type TimestampNanos = u64;
pub type UserId = Principal;
