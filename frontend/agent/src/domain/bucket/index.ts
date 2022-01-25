export type UploadChunkResponse =
    | "success"
    | "file_already_exists"
    | "file_too_big"
    | "chunk_already_exists"
    | "chunk_index_too_high"
    | "chunk_size_mismatch"
    | "allowance_reached"
    | "user_not_found"
    | "hash_mismatch"
    | "full";

export type DeleteFileResponse =
    | "success"
    | "not_authorized"
    | "not_found";
