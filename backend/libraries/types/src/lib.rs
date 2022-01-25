use candid::Principal;

mod canister_wasm;
mod cycles;
mod file;
mod file_status;
mod timestamped;
mod version;

pub use canister_wasm::*;
pub use cycles::*;
pub use file::*;
pub use file_status::*;
pub use timestamped::*;
pub use version::*;

pub type AccessorId = Principal;
pub type CanisterId = Principal;
pub type FileId = u128;
pub type Hash = [u8; 32];
pub type Milliseconds = u64;
pub type TimestampMillis = u64;
pub type TimestampNanos = u64;
pub type UserId = Principal;
