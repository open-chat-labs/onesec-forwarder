use crate::IcpAccount;
use crate::state::tracked_addresses::TrackedAddresses;
use ic_principal::Principal;
use std::collections::HashSet;

mod tracked_addresses;

pub use tracked_addresses::DefaultTrackedAddresses;

pub struct State<T = DefaultTrackedAddresses> {
    tracked_addresses: T,
    whitelisted_callers: HashSet<Principal>,
}

impl<T: TrackedAddresses> State<T> {
    pub fn new(tracked_addresses: T, whitelisted_callers: HashSet<Principal>) -> Self {
        State {
            tracked_addresses,
            whitelisted_callers,
        }
    }

    pub fn enable_forwarding(&mut self, evm_address: String, icp_account: IcpAccount) {
        self.tracked_addresses.push(icp_account, evm_address);
    }

    pub fn is_forwarding(&self, evm_address: &str) -> Option<IcpAccount> {
        self.tracked_addresses.get(evm_address)
    }

    pub fn caller_is_whitelisted(&self, caller: &Principal) -> bool {
        self.whitelisted_callers.contains(caller)
    }
}
