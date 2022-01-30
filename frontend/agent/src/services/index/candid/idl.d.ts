import type { IDL } from "@dfinity/candid";
import {
    _SERVICE,
    AllocatedBucketResponse,
    ReferenceCountsResponse,
    UserResponse,
} from "./types";
export {
    _SERVICE as IndexService,
    AllocatedBucketResponse as CandidAllocatedBucketResponse,
    ReferenceCountsResponse as CandidReferenceCountsResponse,
    UserResponse as CandidUserResponse,
};

export const idlFactory: IDL.InterfaceFactory;
