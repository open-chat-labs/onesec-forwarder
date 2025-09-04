use onesec_forwarder_canister_core as lib;
use onesec_forwarder_canister_types::*;

mod enable_forwarding;
mod forwarding_addresses;
mod init;
mod is_forwarding_address;
mod memory;

ic_cdk::export_candid!();
