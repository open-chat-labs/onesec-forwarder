use candid::CandidType;
use ic_cdk::update;
use serde::{Deserialize, Serialize};

#[update]
fn track_address(args: TrackAddressArgs) {
    crate::lib::track_address(args.address);
}

#[derive(CandidType, Serialize, Deserialize)]
pub struct TrackAddressArgs {
    pub address: String,
}
