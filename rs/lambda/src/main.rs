use crate::config::build_config;
use aws_config::BehaviorVersion;
use ic_principal::Principal;
use lambda_runtime::{Error, LambdaEvent, service_fn, tracing};
use onesec_forwarder_constants::IC_API_GATEWAY_URL;
use onesec_forwarder_lambda_canister_client::CanisterClient;
use onesec_forwarder_lambda_core::Runner;
use onesec_forwarder_lambda_evm_rpc_client::EthRpcClient;
use onesec_forwarder_lambda_parameter_store_client::ParameterStoreClient;
use onesec_forwarder_types::EvmChain;
use serde::Deserialize;

mod config;

#[derive(Deserialize)]
struct Args {
    forwarder_canister_id: Option<Principal>,
    minter_canister_id: Option<Principal>,
    alchemy_api_key: Option<String>,
    max_blocks_per_request_ethereum: Option<u32>,
    max_blocks_per_request_arbitrum: Option<u32>,
    max_blocks_per_request_base: Option<u32>,
}

struct Config {
    forwarder_canister_id: Principal,
    minter_canister_id: Principal,
    alchemy_api_key: String,
    max_blocks_per_request_ethereum: u32,
    max_blocks_per_request_arbitrum: u32,
    max_blocks_per_request_base: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    lambda_runtime::run(service_fn(run_async)).await?;
    Ok(())
}

async fn run_async(request: LambdaEvent<Args>) -> Result<(), Error> {
    let aws_sdk_config = aws_config::defaults(BehaviorVersion::latest()).load().await;

    let config = build_config(request.payload)?;

    let max_blocks_per_request_map = [
        (EvmChain::Ethereum, config.max_blocks_per_request_ethereum),
        (EvmChain::Arbitrum, config.max_blocks_per_request_arbitrum),
        (EvmChain::Base, config.max_blocks_per_request_base),
    ]
    .into_iter()
    .collect();

    let forwarder_client = CanisterClient::new(config.forwarder_canister_id, IC_API_GATEWAY_URL);
    let minter_client = CanisterClient::new(config.minter_canister_id, IC_API_GATEWAY_URL);
    let block_heights_store = ParameterStoreClient::new(&aws_sdk_config);
    let eth_rpc_client = EthRpcClient::new(config.alchemy_api_key, max_blocks_per_request_map);

    let runner = Runner::new(
        forwarder_client,
        minter_client.clone(),
        minter_client,
        block_heights_store,
        eth_rpc_client,
    );

    runner.run().await?;
    Ok(())
}
