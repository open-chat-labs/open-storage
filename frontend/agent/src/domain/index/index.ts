import type { Principal } from "@dfinity/principal";

export type AllocatedBucketResponse =
    | AllocatedBucketSuccess
    | AllocatedBucketBucketUnavailable
    | AllowanceExceeded
    | UserNotFound;

export type AllocatedBucketSuccess = {
    kind: "success";
    canisterId: Principal;
    fileId: bigint;
    chunkSize: number;
    projectedAllowance: ProjectedAllowance,
};

export type AllocatedBucketBucketUnavailable = {
    kind: "bucket_unavailable";
};

export type CanForwardResponse =
    | CanForwardSuccess
    | AllowanceExceeded
    | UserNotFound;

export type CanForwardSuccess = {
    kind: "success",
    projectedAllowance: ProjectedAllowance
};

export interface UploadFileResponse {
    canisterId: Principal;
    fileId: bigint;
    pathPrefix: string;
    projectedAllowance: ProjectedAllowance,
}

export type UserResponse = UserRecord | UserNotFound;

export type UserRecord = {
    kind: "user";
    byteLimit: bigint;
    bytesUsed: bigint;
};

export type UserNotFound = {
    kind: "user_not_found";
};

export type AllowanceExceeded = {
    kind: "allowance_exceeded",
    projectedAllowance: ProjectedAllowance
};

export type ProjectedAllowance = {
    byteLimit: bigint,
    bytesUsed: bigint,
    bytesUsedAfterOperation: bigint,
};
