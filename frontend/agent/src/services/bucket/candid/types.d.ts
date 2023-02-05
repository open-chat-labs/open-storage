import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type AccessorId = Principal;
export type CanisterId = Principal;
export type Cycles = bigint;
export interface DeleteFileArgs { 'file_id' : FileId }
export interface DeleteFileFailure {
  'reason' : DeleteFileFailureReason,
  'file_id' : FileId,
}
export type DeleteFileFailureReason = { 'NotFound' : null } |
  { 'NotAuthorized' : null };
export type DeleteFileResponse = { 'NotFound' : null } |
  { 'NotAuthorized' : null } |
  { 'Success' : null };
export interface DeleteFilesArgs { 'file_ids' : Array<FileId> }
export interface DeleteFilesResponse {
  'failures' : Array<DeleteFileFailure>,
  'success' : Array<FileId>,
}
export type FileId = bigint;
export interface FileInfoArgs { 'file_id' : FileId }
export type FileInfoResponse = { 'NotFound' : null } |
  { 'Success' : FileInfoSuccessResult };
export interface FileInfoSuccessResult {
  'is_owner' : boolean,
  'file_hash' : Hash,
  'file_size' : bigint,
}
export interface ForwardFileArgs {
  'accessors' : Array<AccessorId>,
  'file_id' : FileId,
}
export type ForwardFileResponse = { 'NotFound' : null } |
  { 'NotAuthorized' : null } |
  { 'Success' : FileId };
export type Hash = Uint8Array | number[];
export type Milliseconds = bigint;
export type TimestampMillis = bigint;
export type TimestampNanos = bigint;
export interface UploadChunkArgs {
  'accessors' : Array<AccessorId>,
  'chunk_index' : number,
  'hash' : Hash,
  'mime_type' : string,
  'total_size' : bigint,
  'bytes' : Uint8Array | number[],
  'expiry' : [] | [TimestampMillis],
  'chunk_size' : number,
  'file_id' : FileId,
}
export type UploadChunkResponse = { 'ChunkAlreadyExists' : null } |
  { 'Full' : null } |
  { 'ChunkSizeMismatch' : null } |
  { 'FileTooBig' : null } |
  { 'ChunkIndexTooHigh' : null } |
  { 'Success' : null } |
  { 'FileExpired' : null } |
  { 'HashMismatch' : null } |
  { 'FileAlreadyExists' : null } |
  { 'AllowanceExceeded' : null } |
  { 'InvalidFileId' : null } |
  { 'UserNotFound' : null };
export type UserId = Principal;
export interface Version {
  'major' : number,
  'minor' : number,
  'patch' : number,
}
export interface _SERVICE {
  'delete_file' : ActorMethod<[DeleteFileArgs], DeleteFileResponse>,
  'delete_files' : ActorMethod<[DeleteFilesArgs], DeleteFilesResponse>,
  'file_info' : ActorMethod<[FileInfoArgs], FileInfoResponse>,
  'forward_file' : ActorMethod<[ForwardFileArgs], ForwardFileResponse>,
  'upload_chunk_v2' : ActorMethod<[UploadChunkArgs], UploadChunkResponse>,
}
