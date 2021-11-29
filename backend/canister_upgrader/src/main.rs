use canister_client::operations::*;
use canister_client::utils::get_dfx_identity;
use canister_client::CanisterName;
use clap::Parser;
use types::{CanisterId, Version};

#[tokio::main]
async fn main() {
    let opts = Opts::parse();

    let identity = get_dfx_identity(&opts.controller);

    match opts.canister_to_upgrade {
        CanisterName::Index => upgrade_index_canister(identity, opts.url, opts.index_canister_id, opts.version).await,
        CanisterName::Bucket => upgrade_bucket_canister(identity, opts.url, opts.index_canister_id, opts.version).await,
    };
}

#[derive(Parser)]
struct Opts {
    url: String,
    controller: String,
    index_canister_id: CanisterId,
    canister_to_upgrade: CanisterName,
    version: Version,
}
