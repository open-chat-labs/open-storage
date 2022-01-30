import type { AllocatedBucketResponse, ReferenceCountsResponse, UserResponse } from "../../domain/index";

export interface IIndexClient {
    user(): Promise<UserResponse>;
    allocatedBucket(fileHash: Array<number>, fileSize: bigint): Promise<AllocatedBucketResponse>;
    referenceCounts(fileHash: Array<number>): Promise<ReferenceCountsResponse>;
}
