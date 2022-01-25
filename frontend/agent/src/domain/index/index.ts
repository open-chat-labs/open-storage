import type { Principal } from "@dfinity/principal";

export type AllocatedBucketResponse =
    | AllocatedBucketSuccess
    | AllocatedBucketAllowanceReached
    | AllocatedBucketUserNotFound
    | AllocatedBucketBucketUnavailable;

export type AllocatedBucketSuccess = {
    kind: "success",
    canisterId: Principal,
    chunkSize: number,
}

export type AllocatedBucketAllowanceReached = {
    kind: "allowance_reached",
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
}

export type UserResponse = UserRecord | "user_not_found";

export type UserRecord = {
    byteLimit: bigint,
    bytesUsed: bigint,
}
