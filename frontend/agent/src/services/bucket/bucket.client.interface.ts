import type { Principal } from "@dfinity/principal";
import type { DeleteFileResponse, UploadChunkResponse } from "../../domain/bucket";

export interface IBucketClient {
    uploadChunk(
        fileId: bigint,
        hash: Array<number>,
        mimeType: string,
        accessors: Array<Principal>,
        totalSize: bigint,
        chunkSize: number,
        chunkIndex: number,
        bytes: Array<number>): Promise<UploadChunkResponse>;
    deleteFile(fileId: bigint) : Promise<DeleteFileResponse>;
}
