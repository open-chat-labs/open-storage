import type { Principal } from "@dfinity/principal";

export type AllocatedBucketResponse =
    | AllocatedBucketSuccess
    | AllocatedBucketAllowanceExceeded
    | AllocatedBucketUserNotFound
    | AllocatedBucketBucketUnavailable;

export type AllocatedBucketSuccess = {
    kind: "success",
    canisterId: Principal,
    chunkSize: number,
    byteLimit: bigint;
    bytesUsed: bigint;
    bytesUsedAfterUpload: bigint;
}

export type AllocatedBucketAllowanceExceeded = {
    kind: "allowance_exceeded",
    byteLimit: bigint;
    bytesUsed: bigint;
    bytesUsedAfterUpload: bigint;
}

export type AllocatedBucketUserNotFound = {
    kind: "user_not_found",
}

export type AllocatedBucketBucketUnavailable = {
    kind: "bucket_unavailable",
}

export interface UploadFileResponse {
    canisterId: Principal,
    fileId: bigint,
    pathPrefix: string,
    byteLimit: bigint,
    bytesUsed: bigint,
}

export type UserResponse = UserRecord | "user_not_found";

export type UserRecord = {
    byteLimit: bigint,
    bytesUsed: bigint,
}
