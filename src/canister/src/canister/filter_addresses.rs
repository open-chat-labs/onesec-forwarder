use crate::guards::caller_is_whitelisted;
use ic_cdk::query;
use one_sec_deposit_notifier_canister_types::{FilterAddressesArgs, FilterAddressesResult};

#[query(guard = "caller_is_whitelisted")]
fn filter_addresses(args: FilterAddressesArgs) -> FilterAddressesResult {
    let tracked_addresses = args
        .addresses
        .into_iter()
        .filter(|a| crate::lib::is_address_tracked(&a))
        .collect();

    FilterAddressesResult { tracked_addresses }
}
