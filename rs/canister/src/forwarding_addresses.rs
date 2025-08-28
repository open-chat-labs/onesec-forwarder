use crate::guards::caller_is_whitelisted;
use crate::{ForwardingAddressesArgs, ForwardingAddressesResult};
use ic_cdk::query;

#[query(guard = "caller_is_whitelisted")]
fn forwarding_addresses(args: ForwardingAddressesArgs) -> ForwardingAddressesResult {
    let forwarding_addresses = args
        .evm_addresses
        .into_iter()
        .filter_map(|evm| crate::lib::is_forwarding(&evm).map(|icp| (evm, icp)))
        .collect();

    ForwardingAddressesResult {
        forwarding_addresses,
    }
}
