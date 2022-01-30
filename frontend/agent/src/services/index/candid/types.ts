import type { Principal } from '@dfinity/principal';
export type AccessorId = Principal;
export interface AddOrUpdateUsersArgs { 'users' : Array<UserConfig> }
export type AddOrUpdateUsersResponse = { 'Success' : null };
export interface AllocatedBucketAllowanceExceededResult {
  'byte_limit' : bigint,
  'bytes_used_after_upload' : bigint,
  'bytes_used' : bigint,
}
export interface AllocatedBucketArgs {
  'file_hash' : Hash,
  'file_size' : bigint,
}
export type AllocatedBucketResponse = {
    'Success' : AllocatedBucketSuccessResult
  } |
  { 'AllowanceExceeded' : AllocatedBucketAllowanceExceededResult } |
  { 'UserNotFound' : null } |
  { 'BucketUnavailable' : null };
export interface AllocatedBucketSuccessResult {
  'byte_limit' : bigint,
  'canister_id' : CanisterId,
  'bytes_used_after_upload' : bigint,
  'bytes_used' : bigint,
  'chunk_size' : number,
}
export type CanisterId = Principal;
export type Cycles = bigint;
export type FileId = bigint;
export type Hash = Array<number>;
export type Milliseconds = bigint;
export interface ReferenceCount { 'count' : number, 'bucket' : CanisterId }
export interface ReferenceCountsArgs { 'file_hash' : Hash }
export type ReferenceCountsResponse = {
    'Success' : ReferenceCountsSuccessResult
  } |
  { 'UserNotFound' : null };
export interface ReferenceCountsSuccessResult {
  'byte_limit' : bigint,
  'bytes_used' : bigint,
  'reference_counts' : Array<ReferenceCount>,
}
export interface RemoveAccessorArgs { 'accessor_id' : AccessorId }
export type RemoveAccessorResponse = { 'Success' : null };
export interface RemoveUserArgs { 'user_id' : UserId }
export type RemoveUserResponse = { 'Success' : null };
export type TimestampMillis = bigint;
export type TimestampNanos = bigint;
export type UserArgs = {};
export interface UserConfig { 'byte_limit' : bigint, 'user_id' : UserId }
export type UserId = Principal;
export interface UserRecord { 'byte_limit' : bigint, 'bytes_used' : bigint }
export type UserResponse = { 'Success' : UserRecord } |
  { 'UserNotFound' : null };
export interface Version {
  'major' : number,
  'minor' : number,
  'patch' : number,
}
export interface _SERVICE {
  'add_or_update_users' : (arg_0: AddOrUpdateUsersArgs) => Promise<
      AddOrUpdateUsersResponse
    >,
  'allocated_bucket_v2' : (arg_0: AllocatedBucketArgs) => Promise<
      AllocatedBucketResponse
    >,
  'reference_counts' : (arg_0: ReferenceCountsArgs) => Promise<
      ReferenceCountsResponse
    >,
  'remove_accessor' : (arg_0: RemoveAccessorArgs) => Promise<
      RemoveAccessorResponse
    >,
  'remove_user' : (arg_0: RemoveUserArgs) => Promise<RemoveUserResponse>,
  'user' : (arg_0: UserArgs) => Promise<UserResponse>,
}
