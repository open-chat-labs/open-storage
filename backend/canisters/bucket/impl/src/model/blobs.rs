use bucket_canister::upload_chunk::Args as UploadChunkArgs;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};
use types::{AccessorId, BlobId, Hash, TimestampMillis, UserId};
use utils::hasher::hash_bytes;

#[derive(Serialize, Deserialize, Default)]
pub struct Blobs {
    blobs: HashMap<BlobId, BlobReference>,
    pending_blobs: HashMap<BlobId, PendingBlob>,
    reference_counts: ReferenceCounts,
    accessors_map: AccessorsMap,
    // TODO move this to stable memory
    data: HashMap<Hash, ByteBuf>,
}

#[derive(Serialize, Deserialize)]
struct BlobReference {
    creator: UserId,
    accessors: HashSet<AccessorId>,
    hash: Hash,
    created: TimestampMillis,
}

impl Blobs {
    pub fn put_chunk(&mut self, args: PutChunkArgs) -> PutChunkResult {
        if self.blobs.contains_key(&args.blob_id) {
            return PutChunkResult::BlobAlreadyExists;
        }

        let blob_id = args.blob_id;
        let now = args.now;

        let completed_blob: Option<PendingBlob> = match self.pending_blobs.entry(blob_id) {
            Vacant(e) => {
                let pending_blob: PendingBlob = args.into();
                if pending_blob.is_completed() {
                    Some(pending_blob)
                } else {
                    e.insert(pending_blob);
                    None
                }
            }
            Occupied(mut e) => {
                let pending_blob = e.get_mut();
                if !pending_blob.add_chunk(args.chunk_index, args.bytes) {
                    return PutChunkResult::ChunkAlreadyExists;
                }
                if pending_blob.is_completed() {
                    Some(e.remove())
                } else {
                    None
                }
            }
        };

        if let Some(completed_blob) = completed_blob {
            let hash = hash_bytes(&completed_blob.bytes);
            if hash != completed_blob.hash {
                return PutChunkResult::HashMismatch;
            }
            self.insert_completed_blob(blob_id, completed_blob, now);
            PutChunkResult::Complete
        } else {
            PutChunkResult::Success
        }
    }

    pub fn remove_blob_reference(&mut self, user_id: UserId, blob_id: BlobId) -> RemoveBlobReferenceResult {
        if let Occupied(e) = self.blobs.entry(blob_id) {
            if e.get().creator != user_id {
                RemoveBlobReferenceResult::NotAuthorized
            } else {
                let blob_reference = e.remove();
                for accessor_id in blob_reference.accessors.into_iter() {
                    self.accessors_map.unlink(accessor_id, &blob_id);
                }
                if self.reference_counts.decr(blob_reference.hash) == 0 {
                    self.data.remove(&blob_reference.hash);
                    return RemoveBlobReferenceResult::SuccessBlobDeleted;
                }

                RemoveBlobReferenceResult::Success
            }
        } else {
            RemoveBlobReferenceResult::NotFound
        }
    }

    pub fn remove_accessor(&mut self, accessor_id: &AccessorId) {
        if let Some(blob_ids) = self.accessors_map.remove(accessor_id) {
            for blob_id in blob_ids.into_iter() {
                if let Occupied(mut e) = self.blobs.entry(blob_id) {
                    let blob_reference = e.get_mut();
                    blob_reference.accessors.remove(accessor_id);
                    if blob_reference.accessors.is_empty() {
                        if self.reference_counts.decr(blob_reference.hash) == 0 {
                            self.data.remove(&blob_reference.hash);
                        }
                        e.remove();
                    }
                }
            }
        }
    }

    fn insert_completed_blob(&mut self, blob_id: BlobId, completed_blob: PendingBlob, now: TimestampMillis) {
        for accessor_id in completed_blob.accessors.iter() {
            self.accessors_map.link(*accessor_id, blob_id);
        }

        self.blobs.insert(
            blob_id,
            BlobReference {
                creator: completed_blob.creator,
                accessors: completed_blob.accessors,
                hash: completed_blob.hash,
                created: now,
            },
        );
        self.reference_counts.incr(completed_blob.hash);
        self.data.entry(completed_blob.hash).or_insert(completed_blob.bytes);
    }
}

#[derive(Serialize, Deserialize, Default)]
struct ReferenceCounts {
    counts: HashMap<Hash, u32>,
}

impl ReferenceCounts {
    pub fn incr(&mut self, hash: Hash) -> u32 {
        *self
            .counts
            .entry(hash)
            .and_modify(|c| {
                *c += 1;
            })
            .or_insert(1)
    }

    pub fn decr(&mut self, hash: Hash) -> u32 {
        if let Occupied(mut e) = self.counts.entry(hash) {
            let count = e.get_mut();
            if *count > 1 {
                *count -= 1;
                *count
            } else {
                e.remove();
                0
            }
        } else {
            0
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
struct AccessorsMap {
    map: HashMap<AccessorId, HashSet<BlobId>>,
}

impl AccessorsMap {
    pub fn link(&mut self, accessor_id: AccessorId, blob_id: BlobId) {
        self.map.entry(accessor_id).or_default().insert(blob_id);
    }

    pub fn unlink(&mut self, accessor_id: AccessorId, blob_id: &BlobId) {
        if let Occupied(mut e) = self.map.entry(accessor_id) {
            let entry = e.get_mut();
            entry.remove(blob_id);
            if entry.is_empty() {
                e.remove();
            }
        }
    }

    pub fn remove(&mut self, accessor_id: &AccessorId) -> Option<HashSet<BlobId>> {
        self.map.remove(&accessor_id)
    }
}

#[derive(Serialize, Deserialize)]
struct PendingBlob {
    creator: UserId,
    created: TimestampMillis,
    hash: Hash,
    mime_type: String,
    accessors: HashSet<AccessorId>,
    chunk_size: u32,
    total_size: u32,
    remaining_chunks: HashSet<u32>,
    bytes: ByteBuf,
}

impl PendingBlob {
    pub fn add_chunk(&mut self, chunk_index: u32, bytes: ByteBuf) -> bool {
        if self.remaining_chunks.remove(&chunk_index) {
            let start_index = self.chunk_size * chunk_index;
            for (index, byte) in bytes.into_iter().enumerate().map(|(i, b)| (i + start_index as usize, b)) {
                self.bytes[index] = byte;
            }
            true
        } else {
            false
        }
    }

    pub fn is_completed(&self) -> bool {
        self.remaining_chunks.is_empty()
    }
}

pub struct PutChunkArgs {
    creator: UserId,
    blob_id: BlobId,
    hash: Hash,
    mime_type: String,
    accessors: Vec<AccessorId>,
    chunk_index: u32,
    chunk_size: u32,
    total_size: u32,
    bytes: ByteBuf,
    now: TimestampMillis,
}

impl PutChunkArgs {
    pub fn new(creator: UserId, now: TimestampMillis, upload_chunk_args: UploadChunkArgs) -> Self {
        Self {
            creator,
            blob_id: upload_chunk_args.blob_id,
            hash: upload_chunk_args.hash,
            mime_type: upload_chunk_args.mime_type,
            accessors: upload_chunk_args.accessors,
            chunk_index: upload_chunk_args.chunk_index,
            chunk_size: upload_chunk_args.chunk_size,
            total_size: upload_chunk_args.total_size,
            bytes: upload_chunk_args.bytes,
            now,
        }
    }
}

impl From<PutChunkArgs> for PendingBlob {
    fn from(args: PutChunkArgs) -> Self {
        let chunk_count = ((args.total_size - 1) / args.chunk_size) + 1;

        let mut pending_blob = Self {
            creator: args.creator,
            created: args.now,
            hash: args.hash,
            mime_type: args.mime_type,
            accessors: args.accessors.into_iter().collect(),
            chunk_size: args.chunk_size,
            total_size: args.total_size,
            remaining_chunks: (0..chunk_count).into_iter().collect(),
            bytes: ByteBuf::with_capacity(args.total_size as usize),
        };
        pending_blob.add_chunk(args.chunk_index, args.bytes);
        pending_blob
    }
}

pub enum PutChunkResult {
    Success,
    Complete,
    BlobAlreadyExists,
    ChunkAlreadyExists,
    HashMismatch,
}

pub enum RemoveBlobReferenceResult {
    Success,
    SuccessBlobDeleted,
    NotAuthorized,
    NotFound,
}
