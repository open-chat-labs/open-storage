use candid::CandidType;
use serde::Deserialize;
use serde_bytes::ByteBuf;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    blob_id: u128,
    mime_type: String,
    total_chunks: u32,
    index: u32,
    bytes: ByteBuf,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success,
    NotAuthorised,
    BlobAlreadyExists,
    ChunkAlreadyExists,
    ChunkTooBig,
    Full,
}
