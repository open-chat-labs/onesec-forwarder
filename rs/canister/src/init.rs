use crate::lib::DefaultForwardingAddresses;
use crate::memory::get_forwarding_addresses_memory;
use ic_cdk::init;

#[init]
fn init() {
    crate::lib::init(DefaultForwardingAddresses::init(
        get_forwarding_addresses_memory(),
    ));
}
