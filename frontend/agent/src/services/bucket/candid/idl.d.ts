import type { IDL } from "@dfinity/candid";
import {
    _SERVICE,
    UploadChunkResponse,
    DeleteFileResponse,
} from "./types";
export {
    _SERVICE as BucketService,
    UploadChunkResponse as CandidUploadChunkResponse,
    DeleteFileResponse as CandidDeleteFileResponse,
};

export const idlFactory: IDL.InterfaceFactory;
