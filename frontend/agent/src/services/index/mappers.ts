import type {
    CandidAllocatedBucketResponse,
    CandidUserResponse,
} from "./candid/idl";
import type { AllocatedBucketResponse, UserResponse } from "../../domain/index";
import { UnsupportedValueError } from "../../utils/error";

export function allocatedBucketResponse(
    candid: CandidAllocatedBucketResponse
): AllocatedBucketResponse {
    if ("Success" in candid) {
        return {
            kind: "success",
            canisterId: candid.Success.canister_id,
            chunkSize: candid.Success.chunk_size,
            byteLimit: candid.Success.byte_limit,
            bytesUsed: candid.Success.bytes_used,
            bytesUsedAfterUpload: candid.Success.bytes_used_after_upload,
        };
    }
    if ("AllowanceExceeded" in candid) {
        return {
            kind: "allowance_exceeded",
            byteLimit: candid.AllowanceExceeded.byte_limit,
            bytesUsed: candid.AllowanceExceeded.bytes_used,
            bytesUsedAfterUpload: candid.AllowanceExceeded.bytes_used_after_upload,
        };
    }
    if ("UserNotFound" in candid) {
        return {
            kind: "user_not_found",
        };
    }
    if ("BucketUnavailable" in candid) {
        return {
            kind: "bucket_unavailable",
        };
    }
    throw new UnsupportedValueError(
        "Unknown Index.ApiAllocatedBucketResponse type received",
        candid
    );
}

export function userResponse(candid: CandidUserResponse): UserResponse {
    if ("Success" in candid) {
        return {
            byteLimit: candid.Success.byte_limit,
            bytesUsed: candid.Success.bytes_used,
        };
    }
    if ("UserNotFound" in candid) {
        return "user_not_found";
    }
    throw new UnsupportedValueError(
        "Unknown Index.UserResponse type received",
        candid
    );
}
