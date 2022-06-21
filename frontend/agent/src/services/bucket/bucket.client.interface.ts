import type { Principal } from "@dfinity/principal";
import type { DeleteFileResponse, FileInfoResponse, ForwardFileResponse, UploadChunkResponse } from "../../domain/bucket";

export interface IBucketClient {
    uploadChunk(
        fileId: bigint,
        hash: Array<number>,
        mimeType: string,
        accessors: Array<Principal>,
        totalSize: bigint,
        chunkSize: number,
        chunkIndex: number,
        bytes: Array<number>
    ): Promise<UploadChunkResponse>;
    forwardFile(fileId: bigint, accessors: Array<Principal>): Promise<ForwardFileResponse>;
    deleteFile(fileId: bigint): Promise<DeleteFileResponse>;
    fileInfo(fileId: bigint): Promise<FileInfoResponse>;
}
