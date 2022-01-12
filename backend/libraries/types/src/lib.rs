use candid::Principal;

mod blob_reference;
mod blob_status;
mod canister_wasm;
mod cycles;
mod timestamped;
mod version;

pub use blob_reference::*;
pub use blob_status::*;
pub use canister_wasm::*;
pub use cycles::*;
pub use timestamped::*;
pub use version::*;

pub type AccessorId = Principal;
pub type BlobId = u128;
pub type CanisterId = Principal;
pub type Hash = [u8; 32];
pub type Milliseconds = u64;
pub type TimestampMillis = u64;
pub type TimestampNanos = u64;
pub type UserId = Principal;
