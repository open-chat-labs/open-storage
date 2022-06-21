import type { IDL } from "@dfinity/candid";
import {
    _SERVICE,
    AllocatedBucketResponse,
    UserResponse,
} from "./types";
export {
    _SERVICE as IndexService,
    AllocatedBucketResponse as CandidAllocatedBucketResponse,
    UserResponse as CandidUserResponse,
};

export const idlFactory: IDL.InterfaceFactory;
