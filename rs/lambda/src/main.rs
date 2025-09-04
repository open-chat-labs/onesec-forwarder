use aws_config::BehaviorVersion;
use ic_principal::Principal;
use lambda_runtime::{Error, LambdaEvent, service_fn, tracing};
use onesec_forwarder_constants::IC_API_GATEWAY_URL;
use onesec_forwarder_lambda_canister_client::CanisterClient;
use onesec_forwarder_lambda_core::Runner;
use onesec_forwarder_lambda_evm_rpc_client::EthRpcClient;
use onesec_forwarder_lambda_parameter_store_client::ParameterStoreClient;
use serde::Deserialize;

#[derive(Deserialize)]
struct Args {
    forwarder_canister_id: Principal,
    minter_canister_id: Principal,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    lambda_runtime::run(service_fn(run_async)).await?;
    Ok(())
}

async fn run_async(request: LambdaEvent<Args>) -> Result<(), Error> {
    let args = request.payload;
    let aws_sdk_config = aws_config::defaults(BehaviorVersion::latest()).load().await;

    let Ok(alchemy_api_key) = std::env::var("ALCHEMY_API_KEY") else {
        return Err("ALCHEMY_API_KEY environment variable not set".into());
    };

    let forwarder_client = CanisterClient::new(args.forwarder_canister_id, IC_API_GATEWAY_URL);
    let minter_client = CanisterClient::new(args.minter_canister_id, IC_API_GATEWAY_URL);
    let block_heights_store = ParameterStoreClient::new(&aws_sdk_config);
    let eth_rpc_client = EthRpcClient::new(alchemy_api_key);

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
