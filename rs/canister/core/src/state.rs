use crate::IcpAccount;
use crate::state::tracked_addresses::TrackedAddresses;

mod tracked_addresses;

pub use tracked_addresses::DefaultTrackedAddresses;

pub struct State<T = DefaultTrackedAddresses> {
    tracked_addresses: T,
}

impl<T: TrackedAddresses> State<T> {
    pub fn new(tracked_addresses: T) -> Self {
        State { tracked_addresses }
    }

    pub fn enable_forwarding(&mut self, evm_address: String, icp_account: IcpAccount) {
        self.tracked_addresses.push(icp_account, evm_address);
    }

    pub fn is_forwarding(&self, evm_address: &str) -> Option<IcpAccount> {
        self.tracked_addresses.get(evm_address)
    }
}
