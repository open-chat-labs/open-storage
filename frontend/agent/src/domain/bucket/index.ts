export type UploadChunkResponse =
    | "success"
    | "blob_already_exists"
    | "blob_size_mismatch"
    | "blob_too_big"
    | "chunk_already_exists"
    | "chunk_size_mismatch"
    | "allowance_reached"
    | "user_not_found"
    | "hash_mismatch"
    | "full";

export type DeleteBlobResponse =
    | "success"
    | "not_authorized"
    | "not_found";
