import type { HttpAgent } from "@dfinity/agent";
import type { Principal } from "@dfinity/principal";
import type { IBucketClient } from "./bucket.client.interface";
import { idlFactory, BucketService } from "./candid/idl";
import { deleteFileResponse, uploadChunkResponse } from "./mappers";
import { CandidService } from "../candidService";
import type { DeleteFileResponse, UploadChunkResponse } from "../../domain/bucket";

export class BucketClient extends CandidService<BucketService> implements IBucketClient {
    constructor(agent: HttpAgent, canisterId: Principal) {
        super(agent, idlFactory, canisterId);
    }

    uploadChunk(
        fileId: bigint,
        hash: Array<number>,
        mimeType: string,
        accessors: Array<Principal>,
        totalSize: bigint,
        chunkSize: number,
        chunkIndex: number,
        bytes: Array<number>): Promise<UploadChunkResponse> {
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
            }),
            uploadChunkResponse
        );
    }

    deleteFile(fileId: bigint): Promise<DeleteFileResponse> {
        return this.handleResponse(
            this.service.delete_file({ file_id: fileId }),
            deleteFileResponse
        );
    }
}
