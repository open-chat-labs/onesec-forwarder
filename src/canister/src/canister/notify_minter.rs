use crate::guards::caller_is_whitelisted;
use crate::lib::EvmAddress;
use candid::{CandidType, Deserialize};
use ic_cdk::update;
use serde::Serialize;

#[update(guard = "caller_is_whitelisted")]
fn notify_minter(args: NotifyMinterArgs) {
    for address in args.addresses {
        crate::lib::push_address_onto_notify_minter_queue(address);
    }
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct NotifyMinterArgs {
    addresses: Vec<EvmAddress>,
}
