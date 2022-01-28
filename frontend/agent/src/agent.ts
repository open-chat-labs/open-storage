import type { HttpAgent } from "@dfinity/agent";
import type { Principal } from "@dfinity/principal";
import { v1 as uuidv1 } from "uuid";
import type { UploadFileResponse, UserResponse } from "./domain/index";
import { BucketClient } from "./services/bucket/bucket.client";
import { IndexClient } from "./services/index/index.client";
import type { IIndexClient } from "./services/index/index.client.interface";
import { hashBytes } from "./utils/hash";

export type { UploadFileResponse, UserResponse };

export class OpenStorageAgent {
    private readonly agent: HttpAgent;
    private readonly indexClient: IIndexClient;

    constructor(agent: HttpAgent, indexCanisterId: Principal) {
        this.agent = agent;
        this.indexClient = new IndexClient(agent, indexCanisterId);
    }

    user(): Promise<UserResponse> {
        return this.indexClient.user();
    }

    async uploadFile(
        mimeType: string,
        accessors: Array<Principal>,
        bytes: ArrayBuffer,
        onProgress?: (percentComplete: number) => void
    ): Promise<UploadFileResponse> {
        const hash = Array.from(new Uint8Array(hashBytes(bytes)));
        const fileSize = bytes.byteLength;

        const allocatedBucketResponse = await this.indexClient.allocatedBucket(
            hash,
            BigInt(fileSize)
        );

        if (allocatedBucketResponse.kind !== "success") {
            // TODO make this better!
            throw new Error(allocatedBucketResponse.kind);
        }

        const fileId = OpenStorageAgent.newFileId();
        const bucketCanisterId = allocatedBucketResponse.canisterId;
        const chunkSize = allocatedBucketResponse.chunkSize;
        const chunkCount = Math.ceil(fileSize / chunkSize);
        const chunkIndexes = [...Array(chunkCount).keys()];
        const bucketClient = new BucketClient(this.agent, bucketCanisterId);

        let chunksCompleted = 0;

        const promises = chunkIndexes.map(async (chunkIndex) => {
            const start = chunkIndex * chunkSize;
            const end = Math.min(start + chunkSize, fileSize);
            const chunkBytes = Array.from(new Uint8Array(bytes.slice(start, end)));

            let attempt = 0;

            while (attempt++ < 5) {
                try {
                    const chunkResponse = await bucketClient.uploadChunk(
                        fileId,
                        hash,
                        mimeType,
                        accessors,
                        BigInt(fileSize),
                        chunkSize,
                        chunkIndex,
                        chunkBytes
                    );

                    if (chunkResponse === "success") {
                        chunksCompleted++;
                        onProgress?.((100 * chunksCompleted) / chunkCount);
                        return;
                    }
                } catch (e) {
                    console.log("Error uploading chunk " + chunkIndex, e);
                }
            }
            throw new Error("Failed to upload chunk");
        });

        await Promise.all(promises);

        return {
            canisterId: bucketCanisterId,
            fileId,
            pathPrefix: "/files/",
            byteLimit: allocatedBucketResponse.byteLimit,
            bytesUsed: allocatedBucketResponse.bytesUsedAfterUpload,
        };
    }

    private static newFileId(): bigint {
        return BigInt(parseInt(uuidv1().replace(/-/g, ""), 16));
    }
}
