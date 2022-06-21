import type { Principal } from '@dfinity/principal';
export type AccessorId = Principal;
export interface AddOrUpdateUsersArgs { 'users' : Array<UserConfig> }
export type AddOrUpdateUsersResponse = { 'Success' : null };
export interface AllocatedBucketArgs {
  'file_hash' : Hash,
  'file_size' : bigint,
}
export type AllocatedBucketResponse = {
    'Success' : AllocatedBucketSuccessResult
  } |
  { 'AllowanceExceeded' : ProjectedAllowance } |
  { 'UserNotFound' : null } |
  { 'BucketUnavailable' : null };
export interface AllocatedBucketSuccessResult {
  'byte_limit' : bigint,
  'canister_id' : CanisterId,
  'bytes_used_after_upload' : bigint,
  'bytes_used' : bigint,
  'projected_allowance' : ProjectedAllowance,
  'chunk_size' : number,
}
export interface CanForwardArgs { 'file_hash' : Hash, 'file_size' : bigint }
export type CanForwardResponse = { 'Success' : ProjectedAllowance } |
  { 'AllowanceExceeded' : ProjectedAllowance } |
  { 'UserNotFound' : null };
export type CanisterId = Principal;
export type Cycles = bigint;
export type FileId = bigint;
export type Hash = Array<number>;
export type Milliseconds = bigint;
export interface ProjectedAllowance {
  'bytes_used_after_operation' : bigint,
  'byte_limit' : bigint,
  'bytes_used_after_upload' : bigint,
  'bytes_used' : bigint,
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
  'can_forward' : (arg_0: CanForwardArgs) => Promise<CanForwardResponse>,
  'remove_accessor' : (arg_0: RemoveAccessorArgs) => Promise<
      RemoveAccessorResponse
    >,
  'remove_user' : (arg_0: RemoveUserArgs) => Promise<RemoveUserResponse>,
  'user' : (arg_0: UserArgs) => Promise<UserResponse>,
}
