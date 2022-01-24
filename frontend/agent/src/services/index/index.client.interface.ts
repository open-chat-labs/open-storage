import type { AllocatedBucketResponse, UserResponse } from "../../domain/index";

export interface IIndexClient {
    user(): Promise<UserResponse>;
    allocatedBucket(blobHash: Array<number>, blobSize: bigint): Promise<AllocatedBucketResponse>;
}
