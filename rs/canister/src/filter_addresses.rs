use crate::guards::caller_is_whitelisted;
use crate::{FilterAddressesArgs, FilterAddressesResult};
use ic_cdk::query;

#[query(guard = "caller_is_whitelisted")]
fn filter_addresses(args: FilterAddressesArgs) -> FilterAddressesResult {
    let tracked_addresses = args
        .addresses
        .into_iter()
        .filter(|a| crate::lib::is_address_tracked(&a))
        .collect();

    FilterAddressesResult { tracked_addresses }
}
