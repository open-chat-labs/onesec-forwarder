use crate::lib::DefaultForwardingAddresses;
use crate::memory::get_forwarding_addresses_memory;
use ic_cdk::{init, post_upgrade};

#[init]
fn init() {
    init_state();
}

#[post_upgrade]
fn post_upgrade() {
    init_state();
}

fn init_state() {
    crate::lib::init(DefaultForwardingAddresses::init(
        get_forwarding_addresses_memory(),
    ));
}
