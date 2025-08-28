use onesec_forwarder_types::*;
use std::collections::HashMap;

pub struct Runner<
    F: OneSecForwarderClient,
    M: OneSecMinterClient,
    T: TokenContractAddressesReader,
    B: NextBlockHeightDb,
    E: EthRpcClient,
> {
    onesec_forwarder_client: F,
    onesec_minter_client: M,
    contract_addresses_reader: T,
    next_block_height_db: B,
    eth_rpc_client: E,
}

impl<
    F: OneSecForwarderClient,
    M: OneSecMinterClient,
    T: TokenContractAddressesReader,
    B: NextBlockHeightDb,
    E: EthRpcClient,
> Runner<F, M, T, B, E>
{
    pub fn new(
        onesec_forwarder_client: F,
        onesec_minter_client: M,
        contract_addresses_reader: T,
        next_block_height_db: B,
        eth_rpc_client: E,
    ) -> Self {
        Runner {
            onesec_forwarder_client,
            onesec_minter_client,
            contract_addresses_reader,
            next_block_height_db,
            eth_rpc_client,
        }
    }

    pub async fn run(mut self) -> Result<(), String> {
        let start_block_height = self.next_block_height_db.get().await?;
        let end_block_height = start_block_height + 4;
        let contract_addresses = self.contract_addresses_reader.get().await?;

        let recipients = self
            .eth_rpc_client
            .get_recipients(
                start_block_height,
                end_block_height,
                contract_addresses
                    .iter()
                    .map(|(_, c)| c.address.clone())
                    .collect(),
            )
            .await?;

        if !recipients.is_empty() {
            let forwarding_addresses = self
                .onesec_forwarder_client
                .forwarding_addresses(
                    recipients
                        .iter()
                        .map(|r| r.recipient_address.address.clone())
                        .collect(),
                )
                .await?;

            if !forwarding_addresses.is_empty() {
                let token_lookup: HashMap<_, _> = contract_addresses
                    .into_iter()
                    .map(|(t, c)| (c.address, t))
                    .collect();

                for recipient in recipients {
                    if let Some(icp_account) = forwarding_addresses
                        .get(&recipient.recipient_address.address)
                        .cloned()
                        && let Some(token) = token_lookup.get(&recipient.contract_address).copied()
                    {
                        self.onesec_minter_client
                            .forward_evm_to_icp(token, recipient.recipient_address, icp_account)
                            .await?;
                    }
                }
            }
        }

        self.next_block_height_db.set(end_block_height + 1).await?;
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
    fn get(&self) -> impl Future<Output = Result<Vec<(Token, EvmAddress)>, String>>;
}

pub trait NextBlockHeightDb {
    fn get(&self) -> impl Future<Output = Result<u64, String>>;
    fn set(&mut self, height: u64) -> impl Future<Output = Result<(), String>>;
}

pub trait EthRpcClient {
    fn get_recipients(
        &self,
        from_block: u64,
        to_block: u64,
        contract_addresses: Vec<String>,
    ) -> impl Future<Output = Result<Vec<RecipientContractAddress>, String>>;
}

pub struct RecipientContractAddress {
    pub recipient_address: EvmAddress,
    pub contract_address: String,
}
