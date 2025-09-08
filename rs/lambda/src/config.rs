use crate::{Args, Config};
use ic_principal::Principal;
use std::str::FromStr;

pub fn build_config(args: Args) -> Result<Config, String> {
    let forwarder_canister_id = match args.forwarder_canister_id {
        Some(id) => id,
        None => get_principal_from_env_var("FORWARDER_CANISTER_ID")?,
    };

    let minter_canister_id = match args.minter_canister_id {
        Some(id) => id,
        None => get_principal_from_env_var("MINTER_CANISTER_ID")?,
    };

    let alchemy_api_key = match args.alchemy_api_key {
        Some(api_key) => api_key,
        None => get_env_variable("ALCHEMY_API_KEY")?,
    };

    let max_blocks_per_request_ethereum = match args.max_blocks_per_request_ethereum {
        Some(blocks) => blocks,
        None => get_u32_from_env_var("MAX_BLOCKS_PER_REQUEST_ETHEREUM")?,
    };

    let max_blocks_per_request_arbitrum = match args.max_blocks_per_request_arbitrum {
        Some(blocks) => blocks,
        None => get_u32_from_env_var("MAX_BLOCKS_PER_REQUEST_ARBITRUM")?,
    };

    let max_blocks_per_request_base = match args.max_blocks_per_request_base {
        Some(blocks) => blocks,
        None => get_u32_from_env_var("MAX_BLOCKS_PER_REQUEST_BASE")?,
    };

    Ok(Config {
        forwarder_canister_id,
        minter_canister_id,
        alchemy_api_key,
        max_blocks_per_request_ethereum,
        max_blocks_per_request_arbitrum,
        max_blocks_per_request_base,
    })
}

fn get_principal_from_env_var(name: &str) -> Result<Principal, String> {
    let env_var = get_env_variable(name)?;
    Principal::from_text(env_var).map_err(|e| format!("{name} is not a valid principal: {e}"))
}

fn get_u32_from_env_var(name: &str) -> Result<u32, String> {
    let env_var = get_env_variable(name)?;
    u32::from_str(&env_var).map_err(|e| format!("{name} is not a valid integer: {e}"))
}

fn get_env_variable(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|_| format!("{name} environment variable not set"))
}
