export const idlFactory = ({ IDL }) => {
  const UserId = IDL.Principal;
  const UserConfig = IDL.Record({
    'byte_limit' : IDL.Nat64,
    'user_id' : UserId,
  });
  const AddOrUpdateUsersArgs = IDL.Record({ 'users' : IDL.Vec(UserConfig) });
  const AddOrUpdateUsersResponse = IDL.Variant({ 'Success' : IDL.Null });
  const Hash = IDL.Vec(IDL.Nat8);
  const AllocatedBucketArgs = IDL.Record({
    'file_hash' : Hash,
    'file_size' : IDL.Nat64,
  });
  const CanisterId = IDL.Principal;
  const ProjectedAllowance = IDL.Record({
    'bytes_used_after_operation' : IDL.Nat64,
    'byte_limit' : IDL.Nat64,
    'bytes_used_after_upload' : IDL.Nat64,
    'bytes_used' : IDL.Nat64,
  });
  const AllocatedBucketSuccessResult = IDL.Record({
    'byte_limit' : IDL.Nat64,
    'canister_id' : CanisterId,
    'bytes_used_after_upload' : IDL.Nat64,
    'bytes_used' : IDL.Nat64,
    'projected_allowance' : ProjectedAllowance,
    'chunk_size' : IDL.Nat32,
  });
  const AllocatedBucketResponse = IDL.Variant({
    'Success' : AllocatedBucketSuccessResult,
    'AllowanceExceeded' : ProjectedAllowance,
    'UserNotFound' : IDL.Null,
    'BucketUnavailable' : IDL.Null,
  });
  const CanForwardArgs = IDL.Record({
    'file_hash' : Hash,
    'file_size' : IDL.Nat64,
  });
  const CanForwardResponse = IDL.Variant({
    'Success' : ProjectedAllowance,
    'AllowanceExceeded' : ProjectedAllowance,
    'UserNotFound' : IDL.Null,
  });
  const AccessorId = IDL.Principal;
  const RemoveAccessorArgs = IDL.Record({ 'accessor_id' : AccessorId });
  const RemoveAccessorResponse = IDL.Variant({ 'Success' : IDL.Null });
  const RemoveUserArgs = IDL.Record({ 'user_id' : UserId });
  const RemoveUserResponse = IDL.Variant({ 'Success' : IDL.Null });
  const UserArgs = IDL.Record({});
  const UserRecord = IDL.Record({
    'byte_limit' : IDL.Nat64,
    'bytes_used' : IDL.Nat64,
  });
  const UserResponse = IDL.Variant({
    'Success' : UserRecord,
    'UserNotFound' : IDL.Null,
  });
  return IDL.Service({
    'add_or_update_users' : IDL.Func(
        [AddOrUpdateUsersArgs],
        [AddOrUpdateUsersResponse],
        [],
      ),
    'allocated_bucket_v2' : IDL.Func(
        [AllocatedBucketArgs],
        [AllocatedBucketResponse],
        ['query'],
      ),
    'can_forward' : IDL.Func([CanForwardArgs], [CanForwardResponse], ['query']),
    'remove_accessor' : IDL.Func(
        [RemoveAccessorArgs],
        [RemoveAccessorResponse],
        [],
      ),
    'remove_user' : IDL.Func([RemoveUserArgs], [RemoveUserResponse], []),
    'user' : IDL.Func([UserArgs], [UserResponse], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
