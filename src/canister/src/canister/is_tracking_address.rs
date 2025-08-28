use candid::CandidType;
use ic_cdk::query;
use serde::{Deserialize, Serialize};

#[query]
fn is_tracking_address(args: IsTrackingAddressArgs) -> bool {
    crate::lib::is_address_tracked(&args.address)
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct IsTrackingAddressArgs {
    pub address: String,
}
