import type { HttpAgent } from "@dfinity/agent";
import type { Principal } from "@dfinity/principal";
import { idlFactory, IndexService } from "./candid/idl";
import type { IIndexClient } from "./index.client.interface";
import { allocatedBucketResponse, canForwardResponse, userResponse } from "./mappers";
import { CandidService } from "../candidService";
import type { AllocatedBucketResponse, CanForwardResponse, UserResponse } from "../../domain/index";

export class IndexClient extends CandidService<IndexService> implements IIndexClient {
    constructor(agent: HttpAgent, canisterId: Principal) {
        super(agent, idlFactory, canisterId);
    }

    user(): Promise<UserResponse> {
        return this.handleResponse(this.service.user({}), userResponse);
    }

    allocatedBucket(fileHash: Array<number>, fileSize: bigint): Promise<AllocatedBucketResponse> {
        return this.handleResponse(
            this.service.allocated_bucket_v2({ file_hash: fileHash, file_size: fileSize }),
            allocatedBucketResponse
        );
    }

    canForward(fileHash: Array<number>, fileSize: bigint): Promise<CanForwardResponse> {
        return this.handleResponse(
            this.service.can_forward({ file_hash: fileHash, file_size: fileSize }),
            canForwardResponse
        );
    }
}
