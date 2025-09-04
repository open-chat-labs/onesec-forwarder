use onesec_forwarder_canister_core as lib;
use onesec_forwarder_canister_types::*;

mod enable_forwarding;
mod forwarding_addresses;
mod init;
mod is_forwarding;
mod memory;

fn main() {}

ic_cdk::export_candid!();
