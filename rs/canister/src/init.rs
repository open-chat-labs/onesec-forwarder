use crate::lib::DefaultTrackedAddresses;
use crate::memory::get_forwarding_addresses_memory;
use ic_cdk::init;

#[init]
fn init() {
    crate::lib::init(DefaultTrackedAddresses::init(
        get_forwarding_addresses_memory(),
    ));
}
