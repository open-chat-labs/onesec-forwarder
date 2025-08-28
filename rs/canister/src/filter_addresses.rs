use crate::guards::caller_is_whitelisted;
use crate::{FilterAddressesArgs, FilterAddressesResult};
use ic_cdk::query;

#[query(guard = "caller_is_whitelisted")]
fn filter_addresses(args: FilterAddressesArgs) -> FilterAddressesResult {
    let forwarding_addresses = args
        .evm_addresses
        .into_iter()
        .filter(|a| crate::lib::is_forwarding(a))
        .collect();

    FilterAddressesResult {
        forwarding_addresses,
    }
}
