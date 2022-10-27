use crate::model::stable_blob_storage::StableBlobStorage;
use crate::{calc_chunk_count, MAX_BLOB_SIZE_BYTES};
use bucket_canister::upload_chunk_v2::Args as UploadChunkArgs;
use candid::Principal;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use std::cmp::Ordering;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};
use types::{AccessorId, FileAdded, FileId, FileRemoved, Hash, TimestampMillis, UserId};
use utils::hasher::hash_bytes;

#[derive(Serialize, Deserialize, Default)]
pub struct Files {
    files: HashMap<FileId, File>,
    pending_files: HashMap<FileId, PendingFile>,
    reference_counts: ReferenceCounts,
    accessors_map: AccessorsMap,
    #[serde(alias = "blobs")]
    blobs_old: HashMap<Hash, ByteBuf>,
    #[serde(skip_deserializing)]
    blobs: StableBlobStorage,
    bytes_used: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct File {
    pub owner: UserId,
    pub created: TimestampMillis,
    pub accessors: HashSet<AccessorId>,
    pub hash: Hash,
    pub mime_type: String,
}

impl File {
    pub fn can_be_removed_by(&self, principal: Principal) -> bool {
        // TODO accessors should have roles rather than always being allowed to remove files
        self.owner == principal || self.accessors.contains(&principal)
    }
}

impl Files {
    pub fn migrate_to_stable_storage(&mut self) {
        const BATCH_SIZE: usize = 50;
        let keys_batch: Vec<_> = self.blobs_old.keys().take(BATCH_SIZE).copied().collect();
        for (k, v) in keys_batch.iter().flat_map(|k| self.blobs_old.remove_entry(k)) {
            self.blobs.insert(k, v.into_vec());
        }
    }

    pub fn get(&self, file_id: &FileId) -> Option<&File> {
        self.files.get(file_id)
    }

    pub fn pending_file(&self, file_id: &FileId) -> Option<&PendingFile> {
        self.pending_files.get(file_id)
    }

    pub fn blob_bytes(&self, hash: &Hash) -> Option<Vec<u8>> {
        self.blobs.get(hash)
    }

    pub fn owner(&self, file_id: &FileId) -> Option<UserId> {
        self.files
            .get(file_id)
            .map(|f| f.owner)
            .or_else(|| self.pending_files.get(file_id).map(|f| f.owner))
    }

    pub fn put_chunk(&mut self, args: PutChunkArgs) -> PutChunkResult {
        if args.total_size > MAX_BLOB_SIZE_BYTES {
            return PutChunkResult::FileTooBig(MAX_BLOB_SIZE_BYTES);
        }

        if self.files.contains_key(&args.file_id) {
            return PutChunkResult::FileAlreadyExists;
        }

        let file_id = args.file_id;
        let now = args.now;
        let mut file_added = None;

        let completed_file: Option<PendingFile> = match self.pending_files.entry(file_id) {
            Vacant(e) => {
                file_added = Some(FileAdded {
                    owner: args.owner,
                    file_id,
                    hash: args.hash,
                    size: args.total_size,
                });
                let pending_file: PendingFile = args.into();
                if pending_file.is_completed() {
                    Some(pending_file)
                } else {
                    e.insert(pending_file);
                    None
                }
            }
            Occupied(mut e) => {
                let pending_file = e.get_mut();
                match pending_file.add_chunk(args.chunk_index, args.bytes) {
                    AddChunkResult::Success => {}
                    AddChunkResult::ChunkIndexTooHigh => return PutChunkResult::ChunkIndexTooHigh,
                    AddChunkResult::ChunkAlreadyExists => return PutChunkResult::ChunkAlreadyExists,
                    AddChunkResult::ChunkSizeMismatch(m) => return PutChunkResult::ChunkSizeMismatch(m),
                }
                if pending_file.is_completed() {
                    Some(e.remove())
                } else {
                    None
                }
            }
        };

        let mut file_completed = false;
        if let Some(completed_file) = completed_file {
            let hash = hash_bytes(&completed_file.bytes);
            if hash != completed_file.hash {
                return PutChunkResult::HashMismatch(HashMismatch {
                    provided_hash: completed_file.hash,
                    actual_hash: hash,
                    chunk_count: completed_file.chunk_count(),
                });
            }
            self.insert_completed_file(file_id, completed_file, now);
            file_completed = true;
        }

        PutChunkResult::Success(PutChunkResultSuccess {
            file_completed,
            file_added,
        })
    }

    pub fn remove(&mut self, caller: Principal, file_id: FileId) -> RemoveFileResult {
        if let Occupied(e) = self.files.entry(file_id) {
            if e.get().can_be_removed_by(caller) {
                let file = e.remove();
                for accessor_id in file.accessors.iter() {
                    self.accessors_map.unlink(*accessor_id, &file_id);
                }

                let mut blob_deleted = false;
                if self.reference_counts.decr(file.hash) == 0 {
                    self.remove_blob(&file.hash);
                    blob_deleted = true;
                }

                RemoveFileResult::Success(FileRemoved {
                    file_id,
                    owner: file.owner,
                    hash: file.hash,
                    blob_deleted,
                })
            } else {
                RemoveFileResult::NotAuthorized
            }
        } else {
            RemoveFileResult::NotFound
        }
    }

    pub fn forward(
        &mut self,
        caller: UserId,
        file_id: FileId,
        new_file_id: FileId,
        accessors: HashSet<UserId>,
        now: TimestampMillis,
    ) -> ForwardFileResult {
        let (file, size) = match self.file_and_size(&file_id) {
            Some((f, s)) => (f, s),
            None => return ForwardFileResult::NotFound,
        };

        if file.owner == caller || file.accessors.contains(&caller) {
            let hash = file.hash;

            self.accessors_map.link_many(caller, accessors.iter().copied(), new_file_id);
            self.reference_counts.incr(hash);

            let new_file = File {
                owner: caller,
                created: now,
                accessors,
                hash,
                mime_type: file.mime_type,
            };

            if self.files.insert(new_file_id, new_file).is_none() {
                ForwardFileResult::Success(FileAdded {
                    file_id: new_file_id,
                    owner: caller,
                    hash,
                    size,
                })
            } else {
                // There should never be a file_id clash
                unreachable!();
            }
        } else {
            ForwardFileResult::NotAuthorized
        }
    }

    pub fn remove_pending_file(&mut self, file_id: &FileId) -> bool {
        self.pending_files.remove(file_id).is_some()
    }

    pub fn remove_accessor(&mut self, accessor_id: &AccessorId) -> Vec<FileRemoved> {
        let mut files_removed = Vec::new();

        if let Some(file_ids) = self.accessors_map.remove(accessor_id) {
            for file_id in file_ids {
                let mut blob_to_delete = None;
                if let Occupied(mut e) = self.files.entry(file_id) {
                    let file = e.get_mut();
                    file.accessors.remove(accessor_id);
                    if file.accessors.is_empty() {
                        let delete_blob = self.reference_counts.decr(file.hash) == 0;
                        if delete_blob {
                            blob_to_delete = Some(file.hash);
                        }
                        let file = e.remove();
                        files_removed.push(FileRemoved {
                            file_id,
                            owner: file.owner,
                            hash: file.hash,
                            blob_deleted: delete_blob,
                        });
                    }
                }

                if let Some(blob_to_delete) = blob_to_delete {
                    self.remove_blob(&blob_to_delete);
                }
            }
        }

        files_removed
    }

    pub fn update_owner(&mut self, file_id: &FileId, new_owner: UserId) -> bool {
        if let Some(file) = self.files.get_mut(file_id) {
            file.owner = new_owner;
            true
        } else {
            false
        }
    }

    pub fn update_accessor_id(&mut self, old_accessor_id: AccessorId, new_accessor_id: AccessorId) {
        if let Some(files) = self.accessors_map.map.remove(&old_accessor_id) {
            for file_id in files.iter() {
                if let Some(file) = self.files.get_mut(file_id) {
                    if file.accessors.remove(&old_accessor_id) {
                        file.accessors.insert(new_accessor_id);
                    }
                }
            }

            self.accessors_map.map.insert(new_accessor_id, files);
        }
    }

    pub fn contains_hash(&self, hash: &Hash) -> bool {
        self.blobs.exists(hash)
    }

    pub fn data_size(&self, hash: &Hash) -> Option<u64> {
        self.blobs.data_size(hash)
    }

    pub fn bytes_used(&self) -> u64 {
        self.bytes_used
    }

    pub fn metrics(&self) -> Metrics {
        Metrics {
            file_count: self.files.len() as u64,
            blob_count: self.blobs.len(),
            blobs_to_migrate: self.blobs_old.len(),
        }
    }

    fn insert_completed_file(&mut self, file_id: FileId, completed_file: PendingFile, now: TimestampMillis) {
        self.accessors_map
            .link_many(completed_file.owner, completed_file.accessors.iter().copied(), file_id);

        self.reference_counts.incr(completed_file.hash);
        self.add_blob_if_not_exists(completed_file.hash, completed_file.bytes.into_vec());

        self.files.insert(
            file_id,
            File {
                owner: completed_file.owner,
                created: now,
                accessors: completed_file.accessors,
                hash: completed_file.hash,
                mime_type: completed_file.mime_type,
            },
        );
    }

    fn add_blob_if_not_exists(&mut self, hash: Hash, bytes: Vec<u8>) {
        if !self.blobs.exists(&hash) {
            self.bytes_used = self
                .bytes_used
                .checked_add(bytes.len() as u64)
                .expect("'bytes_used' overflowed");

            self.blobs.insert(hash, bytes);
        }
    }

    fn remove_blob(&mut self, hash: &Hash) {
        if let Some(size) = self.blobs.data_size(hash) {
            self.blobs.remove(hash);
            self.bytes_used = self.bytes_used.checked_sub(size as u64).expect("'bytes used' underflowed");
        }
    }

    fn file_and_size(&self, file_id: &FileId) -> Option<(File, u64)> {
        let file = self.get(file_id)?;
        let size = self.blobs.get(&file.hash).map(|b| b.len() as u64)?;

        Some((file.clone(), size))
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
                return *count;
            } else {
                e.remove();
            }
        }
        0
    }
}

#[derive(Serialize, Deserialize, Default)]
struct AccessorsMap {
    map: HashMap<AccessorId, HashSet<FileId>>,
}

impl AccessorsMap {
    pub fn link_many(&mut self, owner: UserId, accessors: impl Iterator<Item = AccessorId>, file_id: FileId) {
        self.link(owner, file_id);

        for accessor in accessors {
            self.link(accessor, file_id);
        }
    }

    pub fn link(&mut self, accessor_id: AccessorId, file_id: FileId) {
        self.map.entry(accessor_id).or_default().insert(file_id);
    }

    pub fn unlink(&mut self, accessor_id: AccessorId, file_id: &FileId) {
        if let Occupied(mut e) = self.map.entry(accessor_id) {
            let entry = e.get_mut();
            entry.remove(file_id);
            if entry.is_empty() {
                e.remove();
            }
        }
    }

    pub fn remove(&mut self, accessor_id: &AccessorId) -> Option<HashSet<FileId>> {
        self.map.remove(accessor_id)
    }
}

#[derive(Serialize, Deserialize)]
pub struct PendingFile {
    pub owner: UserId,
    pub created: TimestampMillis,
    pub hash: Hash,
    pub mime_type: String,
    pub accessors: HashSet<AccessorId>,
    pub chunk_size: u32,
    pub total_size: u64,
    pub remaining_chunks: HashSet<u32>,
    pub bytes: ByteBuf,
}

impl PendingFile {
    pub fn add_chunk(&mut self, chunk_index: u32, bytes: ByteBuf) -> AddChunkResult {
        if self.remaining_chunks.remove(&chunk_index) {
            if let Some(expected_chunk_size) = self.expected_chunk_size(chunk_index) {
                let actual_chunk_size = bytes.len() as u32;
                if expected_chunk_size != actual_chunk_size {
                    return AddChunkResult::ChunkSizeMismatch(ChunkSizeMismatch {
                        expected_size: expected_chunk_size,
                        actual_size: actual_chunk_size,
                    });
                }
            } else {
                return AddChunkResult::ChunkIndexTooHigh;
            }

            let start_index = self.chunk_size as usize * chunk_index as usize;
            let end_index = start_index + bytes.len();
            self.bytes[start_index..end_index].copy_from_slice(&bytes);

            AddChunkResult::Success
        } else {
            AddChunkResult::ChunkAlreadyExists
        }
    }

    pub fn chunk_count(&self) -> u32 {
        calc_chunk_count(self.chunk_size, self.total_size)
    }

    pub fn is_completed(&self) -> bool {
        self.remaining_chunks.is_empty()
    }

    fn expected_chunk_size(&self, chunk_index: u32) -> Option<u32> {
        let last_index = self.chunk_count() - 1;
        match chunk_index.cmp(&last_index) {
            Ordering::Equal => Some(((self.total_size - 1) % self.chunk_size as u64) as u32 + 1),
            Ordering::Less => Some(self.chunk_size),
            Ordering::Greater => None,
        }
    }
}

pub enum AddChunkResult {
    Success,
    ChunkAlreadyExists,
    ChunkIndexTooHigh,
    ChunkSizeMismatch(ChunkSizeMismatch),
}

pub struct PutChunkArgs {
    owner: UserId,
    file_id: FileId,
    hash: Hash,
    mime_type: String,
    accessors: Vec<AccessorId>,
    chunk_index: u32,
    chunk_size: u32,
    total_size: u64,
    bytes: ByteBuf,
    now: TimestampMillis,
}

impl PutChunkArgs {
    pub fn new(owner: UserId, upload_chunk_args: UploadChunkArgs, now: TimestampMillis) -> Self {
        Self {
            owner,
            file_id: upload_chunk_args.file_id,
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

impl From<PutChunkArgs> for PendingFile {
    fn from(args: PutChunkArgs) -> Self {
        let chunk_count = calc_chunk_count(args.chunk_size, args.total_size);

        let mut pending_file = Self {
            owner: args.owner,
            created: args.now,
            hash: args.hash,
            mime_type: args.mime_type,
            accessors: args.accessors.into_iter().collect(),
            chunk_size: args.chunk_size,
            total_size: args.total_size,
            remaining_chunks: (0..chunk_count).into_iter().collect(),
            bytes: ByteBuf::from(vec![0; args.total_size as usize]),
        };
        pending_file.add_chunk(args.chunk_index, args.bytes);
        pending_file
    }
}

pub enum PutChunkResult {
    Success(PutChunkResultSuccess),
    FileAlreadyExists,
    FileTooBig(u64),
    ChunkAlreadyExists,
    ChunkIndexTooHigh,
    ChunkSizeMismatch(ChunkSizeMismatch),
    HashMismatch(HashMismatch),
}

pub struct PutChunkResultSuccess {
    pub file_completed: bool,
    pub file_added: Option<FileAdded>,
}

pub enum RemoveFileResult {
    Success(FileRemoved),
    NotAuthorized,
    NotFound,
}

pub enum ForwardFileResult {
    Success(FileAdded),
    NotAuthorized,
    NotFound,
}

pub struct HashMismatch {
    pub provided_hash: Hash,
    pub actual_hash: Hash,
    pub chunk_count: u32,
}

pub struct ChunkSizeMismatch {
    pub expected_size: u32,
    pub actual_size: u32,
}

pub struct Metrics {
    pub file_count: u64,
    pub blob_count: u64,
    pub blobs_to_migrate: usize,
}
