use itertools::Itertools;
use onesec_forwarder_types::*;
use std::collections::{HashMap, HashSet};
use tracing::info;

pub struct Runner<
    F: OneSecForwarderClient,
    M: OneSecMinterClient,
    T: TokenContractAddressesReader,
    B: NextBlockHeightStore,
    E: EthRpcClient,
> {
    onesec_forwarder_client: F,
    onesec_minter_client: M,
    token_contract_addresses_reader: T,
    next_block_height_store: B,
    eth_rpc_client: E,
}

impl<
    F: OneSecForwarderClient,
    M: OneSecMinterClient,
    T: TokenContractAddressesReader,
    B: NextBlockHeightStore,
    E: EthRpcClient,
> Runner<F, M, T, B, E>
{
    pub fn new(
        onesec_forwarder_client: F,
        onesec_minter_client: M,
        token_contract_addresses_reader: T,
        next_block_height_store: B,
        eth_rpc_client: E,
    ) -> Self {
        Runner {
            onesec_forwarder_client,
            onesec_minter_client,
            token_contract_addresses_reader,
            next_block_height_store,
            eth_rpc_client,
        }
    }

    pub async fn run(mut self) -> Result<(), String> {
        info!("Getting token contract addresses...");

        // Get the ERC20 token contract addresses per chain
        let token_contract_addresses = self.token_contract_addresses_reader.get().await?;

        info!(
            ?token_contract_addresses,
            "Finished getting token contract addresses"
        );

        // For each chain, get the list of addresses that have received any of the ERC20 tokens we
        // care about
        let per_chain_results: Vec<(EvmChain, GetRecipientsForChainResult)> =
            futures::future::try_join_all(token_contract_addresses.into_iter().map(
                async |(chain, addresses)| {
                    self.get_recipients_for_chain(chain, addresses)
                        .await
                        .map(|r| (chain, r))
                },
            ))
            .await?;

        self.process_results(&per_chain_results).await?;

        info!("Setting next block heights...");

        // Update the `next_block_height` value for each chain
        for (
            chain,
            GetRecipientsForChainResult {
                next_block_height, ..
            },
        ) in per_chain_results
        {
            self.next_block_height_store
                .set(chain, next_block_height)
                .await?;
        }

        info!("Finished setting next block heights");

        Ok(())
    }

    async fn get_recipients_for_chain(
        &self,
        chain: EvmChain,
        token_contract_addresses: Vec<TokenContractAddress>,
    ) -> Result<GetRecipientsForChainResult, String> {
        info!(?chain, "Getting next block height");

        let next_block_height = self.next_block_height_store.get(chain).await?;

        info!(
            ?chain,
            next_block_height, "Finished getting next block height"
        );

        let token_lookup: HashMap<_, _> = token_contract_addresses
            .iter()
            .map(|a| (a.address.to_lowercase(), a.token))
            .collect();

        info!(?chain, "Getting recipient addresses...");

        let eth_rpc_result = self
            .eth_rpc_client
            .get_recipients(
                chain,
                next_block_height,
                token_contract_addresses
                    .iter()
                    .map(|a| a.address.clone())
                    .collect(),
            )
            .await?;

        let result = GetRecipientsForChainResult {
            recipients: eth_rpc_result
                .recipients
                .into_iter()
                .filter_map(|r| {
                    token_lookup
                        .get(&r.contract_address.to_lowercase())
                        .map(|t| TokenRecipientAddress {
                            token: *t,
                            recipient_address: r.recipient_address,
                        })
                })
                .collect(),
            next_block_height: eth_rpc_result.next_block_height,
        };

        info!(
            ?chain,
            recipients = result.recipients.len(),
            "Finished getting recipient addresses"
        );

        Ok(result)
    }

    async fn process_results(
        &self,
        per_chain_results: &[(EvmChain, GetRecipientsForChainResult)],
    ) -> Result<(), String> {
        // Exit early if there have been no recipients for any of the ERC20 tokens
        if per_chain_results
            .iter()
            .all(|(_, r)| r.recipients.is_empty())
        {
            return Ok(());
        }

        let unique_recipient_addresses: Vec<_> = per_chain_results
            .iter()
            .flat_map(|(_, r)| r.recipients.iter())
            .map(|r| r.recipient_address.clone())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();

        info!(
            unique_recipient_addresses = unique_recipient_addresses.len(),
            "Getting forwarding addresses..."
        );

        // Query the OneSecForwarder canister in batches of 10k to filter out only the addresses
        // that are enabled for forwarding
        let forwarding_addresses: Vec<_> = futures::future::try_join_all(
            unique_recipient_addresses
                .into_iter()
                .chunks(10000)
                .into_iter()
                .map(|batch| {
                    self.onesec_forwarder_client
                        .forwarding_addresses(batch.collect())
                }),
        )
        .await?
        .into_iter()
        .flatten()
        .collect();

        info!(
            forwarding_addresses = forwarding_addresses.len(),
            "Finished getting forwarding addresses"
        );

        if forwarding_addresses.is_empty() {
            return Ok(());
        }

        // For each forwarding address, find the relevent deposits and notify the OneSecMinter
        // canister of each one
        for (evm_address, icp_account) in forwarding_addresses {
            for (chain, GetRecipientsForChainResult { recipients, .. }) in per_chain_results.iter()
            {
                for recipient in recipients
                    .iter()
                    .filter(|r| r.recipient_address == evm_address)
                {
                    info!(?chain, ?evm_address, "Forwarding EVM to ICP...");

                    self.onesec_minter_client
                        .forward_evm_to_icp(
                            recipient.token,
                            EvmAddress {
                                chain: *chain,
                                address: evm_address.clone(),
                            },
                            icp_account.clone(),
                        )
                        .await?;

                    info!(?chain, ?evm_address, "Forwarded EVM to ICP");
                }
            }
        }

        Ok(())
    }
}

pub trait OneSecForwarderClient {
    fn forwarding_addresses(
        &self,
        evm_addresses: Vec<String>,
    ) -> impl Future<Output = Result<HashMap<String, IcpAccount>, String>>;
}

pub trait OneSecMinterClient {
    fn forward_evm_to_icp(
        &self,
        token: Token,
        evm_address: EvmAddress,
        icp_account: IcpAccount,
    ) -> impl Future<Output = Result<(), String>>;
}

pub trait TokenContractAddressesReader {
    fn get(
        &self,
    ) -> impl Future<Output = Result<HashMap<EvmChain, Vec<TokenContractAddress>>, String>>;
}

pub trait NextBlockHeightStore {
    fn get(&self, chain: EvmChain) -> impl Future<Output = Result<u64, String>>;
    fn set(&mut self, chain: EvmChain, height: u64) -> impl Future<Output = Result<(), String>>;
}

pub trait EthRpcClient {
    fn get_recipients(
        &self,
        chain: EvmChain,
        from_block: u64,
        contract_addresses: Vec<String>,
    ) -> impl Future<Output = Result<GetRecipientsResult, String>>;
}

pub struct GetRecipientsResult {
    pub recipients: Vec<RecipientContractAddress>,
    pub next_block_height: u64,
}

pub struct RecipientContractAddress {
    pub contract_address: String,
    pub recipient_address: String,
}

struct GetRecipientsForChainResult {
    next_block_height: u64,
    recipients: Vec<TokenRecipientAddress>,
}

struct TokenRecipientAddress {
    pub token: Token,
    pub recipient_address: String,
}
