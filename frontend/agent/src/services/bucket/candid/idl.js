export const idlFactory = ({ IDL }) => {
  const FileId = IDL.Nat;
  const DeleteFileArgs = IDL.Record({ 'file_id' : FileId });
  const DeleteFileResponse = IDL.Variant({
    'NotFound' : IDL.Null,
    'NotAuthorized' : IDL.Null,
    'Success' : IDL.Null,
  });
  const AccessorId = IDL.Principal;
  const Hash = IDL.Vec(IDL.Nat8);
  const UploadChunkArgs = IDL.Record({
    'accessors' : IDL.Vec(AccessorId),
    'chunk_index' : IDL.Nat32,
    'hash' : Hash,
    'mime_type' : IDL.Text,
    'total_size' : IDL.Nat64,
    'bytes' : IDL.Vec(IDL.Nat8),
    'chunk_size' : IDL.Nat32,
    'file_id' : FileId,
  });
  const UploadChunkResponse = IDL.Variant({
    'ChunkAlreadyExists' : IDL.Null,
    'Full' : IDL.Null,
    'ChunkSizeMismatch' : IDL.Null,
    'FileTooBig' : IDL.Null,
    'ChunkIndexTooHigh' : IDL.Null,
    'Success' : IDL.Null,
    'HashMismatch' : IDL.Null,
    'FileAlreadyExists' : IDL.Null,
    'AllowanceReached' : IDL.Null,
    'UserNotFound' : IDL.Null,
  });
  return IDL.Service({
    'delete_file' : IDL.Func([DeleteFileArgs], [DeleteFileResponse], []),
    'upload_chunk_v2' : IDL.Func([UploadChunkArgs], [UploadChunkResponse], []),
  });
};
export const init = ({ IDL }) => { return []; };
