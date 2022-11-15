export type UploadChunkResponse =
    | "success"
    | "file_already_exists"
    | "file_too_big"
    | "file_expired"
    | "chunk_already_exists"
    | "chunk_index_too_high"
    | "chunk_size_mismatch"
    | "allowance_exceeded"
    | "user_not_found"
    | "hash_mismatch"
    | "full";

export type ForwardFileResponse =
    | { kind: "success", newFileId: bigint }
    | { kind: "not_authorized" }
    | { kind: "file_not_found" };

export type DeleteFileResponse = "success" | "not_authorized" | "file_not_found";

export type FileInfoResponse =
    | FileInfoSuccess
    | { kind: "file_not_found" };

export type FileInfoSuccess = {
    kind: "success",
    isOwner: boolean,
    fileSize: bigint,
    fileHash: Uint8Array,
};
