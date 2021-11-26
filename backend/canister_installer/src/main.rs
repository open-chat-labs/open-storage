use canister_client::operations::install_service_canisters;
use canister_client::utils::get_dfx_identity;
use clap::Parser;
use types::CanisterId;

#[tokio::main]
async fn main() {
    let opts = Opts::parse();

    let identity = get_dfx_identity(&opts.controller);

    install_service_canisters(identity, opts.url, opts.index, opts.test_mode).await;
}

#[derive(Parser)]
struct Opts {
    url: String,
    #[clap(parse(try_from_str))]
    test_mode: bool,
    controller: String,
    index: CanisterId,
}
