import type { CandidUploadChunkResponse, CandidDeleteFileResponse } from "./candid/idl";
import type { UploadChunkResponse, DeleteFileResponse } from "../../domain/bucket";
import { UnsupportedValueError } from "../../utils/error";

export function uploadChunkResponse(candid: CandidUploadChunkResponse): UploadChunkResponse {
    if ("Success" in candid) {
        return "success";
    }
    if ("FileAlreadyExists" in candid) {
        return "file_already_exists";
    }
    if ("FileTooBig" in candid) {
        return "file_too_big";
    }
    if ("ChunkAlreadyExists" in candid) {
        return "chunk_already_exists";
    }
    if ("ChunkSizeMismatch" in candid) {
        return "chunk_size_mismatch";
    }
    if ("ChunkIndexTooHigh" in candid) {
        return "chunk_index_too_high";
    }
    if ("AllowanceExceeded" in candid) {
        return "allowance_exceeded";
    }
    if ("UserNotFound" in candid) {
        return "user_not_found";
    }
    if ("HashMismatch" in candid) {
        return "hash_mismatch";
    }
    if ("Full" in candid) {
        return "full";
    }
    throw new UnsupportedValueError("Unknown Bucket.ApiUploadChunkResponse type received", candid);
}

export function deleteFileResponse(candid: CandidDeleteFileResponse): DeleteFileResponse {
    if ("Success" in candid) {
        return "success";
    }
    if ("NotAuthorized" in candid) {
        return "not_authorized";
    }
    if ("NotFound" in candid) {
        return "not_found";
    }
    throw new UnsupportedValueError("Unknown Bucket.ApiDeleteFileResponse type received", candid);
}
