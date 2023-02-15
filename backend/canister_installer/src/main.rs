use canister_client::operations::install_service_canisters;
use canister_client::utils::get_dfx_identity;
use clap::Parser;
use index_canister::init::CyclesDispenserConfig;
use types::{CanisterId, Cycles};

#[tokio::main]
async fn main() {
    let opts = Opts::parse();

    let identity = get_dfx_identity(&opts.controller);

    let cycles_dispenser_config = cycles_dispenser_config(&opts);

    install_service_canisters(
        identity,
        opts.url,
        opts.user_controller,
        opts.index,
        cycles_dispenser_config,
        opts.test_mode,
    )
    .await;
}

fn cycles_dispenser_config(opts: &Opts) -> Option<CyclesDispenserConfig> {
    let canister_id = opts.cycles_dispenser_canister_id?;
    let min_cycles_balance = opts.min_cycles_balance?;

    Some(CyclesDispenserConfig {
        canister_id,
        min_cycles_balance,
    })
}

#[derive(Parser)]
struct Opts {
    url: String,
    #[clap(parse(try_from_str))]
    test_mode: bool,
    controller: String,
    user_controller: CanisterId,
    index: CanisterId,
    cycles_dispenser_canister_id: Option<CanisterId>,
    min_cycles_balance: Option<Cycles>,
}
