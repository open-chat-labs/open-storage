import type { Principal } from "@dfinity/principal";

export type AllocatedBucketResponse =
    | AllocatedBucketSuccess
    | AllocatedBucketAllowanceExceeded
    | AllocatedBucketBucketUnavailable
    | UserNotFound;

export type AllocatedBucketSuccess = {
    kind: "success";
    canisterId: Principal;
    chunkSize: number;
    byteLimit: bigint;
    bytesUsed: bigint;
    bytesUsedAfterUpload: bigint;
};

export type AllocatedBucketAllowanceExceeded = {
    kind: "allowance_exceeded";
    byteLimit: bigint;
    bytesUsed: bigint;
    bytesUsedAfterUpload: bigint;
};

export type AllocatedBucketBucketUnavailable = {
    kind: "bucket_unavailable";
};

export type UploadFileResponse = {
    canisterId: Principal;
    fileId: bigint;
    pathPrefix: string;
    byteLimit: bigint;
    bytesUsed: bigint;
}

export type ReferenceCountsResponse = ReferenceCountsSuccess | UserNotFound;

export type ReferenceCountsSuccess = {
    kind: "success";
    referenceCounts: Array<ReferenceCount>;
    byteLimit: bigint;
    bytesUsed: bigint;
};

export type ReferenceCount = {
    bucket: Principal;
    count: number;
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
