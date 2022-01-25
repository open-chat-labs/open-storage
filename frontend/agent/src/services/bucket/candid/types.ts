import type { Principal } from '@dfinity/principal';
export type AccessorId = Principal;
export type CanisterId = Principal;
export type Cycles = bigint;
export interface DeleteFileArgs { 'file_id' : FileId }
export type DeleteFileResponse = { 'NotFound' : null } |
  { 'NotAuthorized' : null } |
  { 'Success' : null };
export type FileId = bigint;
export type Hash = Array<number>;
export type Milliseconds = bigint;
export type TimestampMillis = bigint;
export type TimestampNanos = bigint;
export interface UploadChunkArgs {
  'accessors' : Array<AccessorId>,
  'chunk_index' : number,
  'hash' : Hash,
  'mime_type' : string,
  'total_size' : bigint,
  'bytes' : Array<number>,
  'chunk_size' : number,
  'file_id' : FileId,
}
export type UploadChunkResponse = { 'ChunkAlreadyExists' : null } |
  { 'Full' : null } |
  { 'ChunkSizeMismatch' : null } |
  { 'FileTooBig' : null } |
  { 'ChunkIndexTooHigh' : null } |
  { 'Success' : null } |
  { 'HashMismatch' : null } |
  { 'FileAlreadyExists' : null } |
  { 'AllowanceReached' : null } |
  { 'UserNotFound' : null };
export type UserId = Principal;
export interface Version {
  'major' : number,
  'minor' : number,
  'patch' : number,
}
export interface _SERVICE {
  'delete_file' : (arg_0: DeleteFileArgs) => Promise<DeleteFileResponse>,
  'upload_chunk_v2' : (arg_0: UploadChunkArgs) => Promise<UploadChunkResponse>,
}
