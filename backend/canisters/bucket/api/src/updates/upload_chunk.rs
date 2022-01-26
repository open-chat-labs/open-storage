use candid::CandidType;
use serde::Deserialize;
use serde_bytes::ByteBuf;
use std::fmt::{Debug, Formatter};
use types::{AccessorId, FileId, Hash};

#[derive(CandidType, Deserialize)]
pub struct Args {
    pub blob_id: FileId,
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
    AllowanceReached,
    BlobAlreadyExists,
    BlobTooBig,
    ChunkAlreadyExists,
    ChunkIndexTooHigh,
    ChunkSizeMismatch,
    Full,
    HashMismatch,
    UserNotFound,
}

use crate::upload_chunk_v2 as v2;

impl From<Args> for v2::Args {
    fn from(args: Args) -> Self {
        Self {
            file_id: args.blob_id,
            hash: args.hash,
            mime_type: args.mime_type,
            accessors: args.accessors,
            chunk_index: args.chunk_index,
            chunk_size: args.chunk_size,
            total_size: args.total_size,
            bytes: args.bytes,
        }
    }
}

impl From<v2::Response> for Response {
    fn from(response: v2::Response) -> Self {
        match response {
            v2::Response::Success => Self::Success,
            v2::Response::AllowanceReached => Self::AllowanceReached,
            v2::Response::FileAlreadyExists => Self::BlobAlreadyExists,
            v2::Response::FileTooBig => Self::BlobTooBig,
            v2::Response::ChunkAlreadyExists => Self::ChunkAlreadyExists,
            v2::Response::ChunkIndexTooHigh => Self::ChunkIndexTooHigh,
            v2::Response::ChunkSizeMismatch => Self::ChunkSizeMismatch,
            v2::Response::Full => Self::Full,
            v2::Response::HashMismatch => Self::HashMismatch,
            v2::Response::UserNotFound => Self::UserNotFound,
        }
    }
}

impl Debug for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Args")
            .field("blob_id", &self.blob_id)
            .field("hash", &self.hash)
            .field("mime_type", &self.mime_type)
            .field("accessors", &self.accessors)
            .field("chunk_index", &self.chunk_index)
            .field("chunk_size", &self.chunk_size)
            .field("total_size", &self.total_size)
            .field("byte_length", &self.bytes.len())
            .finish()
    }
}
