import type { HttpAgent } from "@dfinity/agent";
import type { Principal } from "@dfinity/principal";
import type { IBucketClient } from "./bucket.client.interface";
import { idlFactory, BucketService } from "./candid/idl";
import { deleteFileResponse, fileInfoResponse, forwardFileResponse, uploadChunkResponse } from "./mappers";
import { CandidService } from "../candidService";
import type { DeleteFileResponse, FileInfoResponse, ForwardFileResponse, UploadChunkResponse } from "../../domain/bucket";

export class BucketClient extends CandidService<BucketService> implements IBucketClient {
    constructor(agent: HttpAgent, canisterId: Principal) {
        super(agent, idlFactory, canisterId);
    }

    uploadChunk(
        fileId: bigint,
        hash: Uint8Array,
        mimeType: string,
        accessors: Array<Principal>,
        totalSize: bigint,
        chunkSize: number,
        chunkIndex: number,
        bytes: Uint8Array,
        expiryTimestampMillis: bigint | undefined,
    ): Promise<UploadChunkResponse> {
        return this.handleResponse(
            this.service.upload_chunk_v2({
                accessors,
                chunk_index: chunkIndex,
                file_id: fileId,
                hash,
                mime_type: mimeType,
                total_size: totalSize,
                bytes,
                chunk_size: chunkSize,
                expiry: expiryTimestampMillis !== undefined ? [expiryTimestampMillis] : []
            }),
            uploadChunkResponse
        );
    }

    forwardFile(fileId: bigint, accessors: Array<Principal>): Promise<ForwardFileResponse> {
        return this.handleResponse(
            this.service.forward_file({ file_id: fileId, accessors }),
            forwardFileResponse
        );
    }

    deleteFile(fileId: bigint): Promise<DeleteFileResponse> {
        return this.handleResponse(
            this.service.delete_file({ file_id: fileId }),
            deleteFileResponse
        );
    }

    fileInfo(fileId: bigint): Promise<FileInfoResponse> {
        return this.handleResponse(
            this.service.file_info({ file_id: fileId }),
            fileInfoResponse
        )
    }
}
