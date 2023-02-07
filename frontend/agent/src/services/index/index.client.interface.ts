import type { AllocatedBucketResponse, CanForwardResponse, UserResponse } from "../../domain/index";

export interface IIndexClient {
    user(): Promise<UserResponse>;
    allocatedBucket(fileHash: Uint8Array, fileSize: bigint, fileIdSeed: bigint | undefined): Promise<AllocatedBucketResponse>;
    canForward(fileHash: Uint8Array, fileSize: bigint): Promise<CanForwardResponse>;
}
