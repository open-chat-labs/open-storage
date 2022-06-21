import type { AllocatedBucketResponse, CanForwardResponse, UserResponse } from "../../domain/index";

export interface IIndexClient {
    user(): Promise<UserResponse>;
    allocatedBucket(fileHash: Array<number>, fileSize: bigint): Promise<AllocatedBucketResponse>;
    canForward(fileHash: Array<number>, fileSize: bigint): Promise<CanForwardResponse>;
}
