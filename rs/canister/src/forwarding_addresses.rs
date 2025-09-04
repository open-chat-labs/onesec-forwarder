use crate::{ForwardingAddressesArgs, ForwardingAddressesResult};
use ic_cdk::query;

#[query]
fn forwarding_addresses(args: ForwardingAddressesArgs) -> ForwardingAddressesResult {
    let forwarding_addresses = args
        .evm_addresses
        .into_iter()
        .filter_map(|evm| crate::lib::is_forwarding_address(&evm).map(|icp| (evm, icp)))
        .collect();

    ForwardingAddressesResult {
        forwarding_addresses,
    }
}
