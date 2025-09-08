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
    forwarder_canister_id: Option<Principal>,
    minter_canister_id: Option<Principal>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    lambda_runtime::run(service_fn(run_async)).await?;
    Ok(())
}

async fn run_async(request: LambdaEvent<Args>) -> Result<(), Error> {
    let aws_sdk_config = aws_config::defaults(BehaviorVersion::latest()).load().await;

    let forwarder_canister_id = match request.payload.forwarder_canister_id {
        Some(id) => id,
        None => get_principal_from_env_var("FORWARDER_CANISTER_ID")?
    };

    let minter_canister_id = match request.payload.minter_canister_id {
        Some(id) => id,
        None => get_principal_from_env_var("MINTER_CANISTER_ID")?
    };

    let alchemy_api_key = get_env_variable("ALCHEMY_API_KEY")?;

    let forwarder_client = CanisterClient::new(forwarder_canister_id, IC_API_GATEWAY_URL);
    let minter_client = CanisterClient::new(minter_canister_id, IC_API_GATEWAY_URL);
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

fn get_principal_from_env_var(name: &str) -> Result<Principal, String> {
    let env_var = get_env_variable(name)?;

    Principal::from_text(env_var).map_err(|e| format!("{name} is not a valid principal: {e}"))
}

fn get_env_variable(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|_| format!("{name} environment variable not set"))
}
