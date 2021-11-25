use candid::CandidType;
use serde::Deserialize;
use serde_bytes::ByteBuf;
use types::{AccessorId, BlobId, Hash};

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub blob_id: BlobId,
    pub hash: Hash,
    pub mime_type: String,
    pub accessors: Vec<AccessorId>,
    pub chunk_index: u32,
    pub chunk_size: u32,
    pub total_size: u64,
    pub bytes: ByteBuf,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success,
    BlobAlreadyExists,
    ChunkAlreadyExists,
    AllowanceReached,
    UserNotFound,
    HashMismatch,
    Full,
}
