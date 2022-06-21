import type { IDL } from "@dfinity/candid";
import {
    _SERVICE,
    AllocatedBucketV2Response,
    UserResponse,
} from "./types";
export {
    _SERVICE as IndexService,
    AllocatedBucketV2Response as CandidAllocatedBucketResponse,
    UserResponse as CandidUserResponse,
};

export const idlFactory: IDL.InterfaceFactory;
