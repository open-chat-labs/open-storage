import type { AllocatedBucketResponse, CanForwardResponse, UserResponse } from "../../domain/index";

export interface IIndexClient {
    user(): Promise<UserResponse>;
    allocatedBucket(fileHash: Uint8Array, fileSize: bigint): Promise<AllocatedBucketResponse>;
    canForward(fileHash: Uint8Array, fileSize: bigint): Promise<CanForwardResponse>;
}
