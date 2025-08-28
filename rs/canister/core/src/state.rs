use crate::state::notify_minter_queue::NotifyMinterQueue;
use crate::state::tracked_addresses::TrackedAddresses;
use crate::{EvmAddress, IcpAccount};
use ic_principal::Principal;
use std::collections::HashSet;

mod notify_minter_queue;
mod tracked_addresses;

pub use notify_minter_queue::DefaultNotifyMinterQueue;
pub use tracked_addresses::DefaultTrackedAddresses;

pub struct State<T = DefaultTrackedAddresses, Q = DefaultNotifyMinterQueue> {
    tracked_addresses: T,
    notify_minter_queue: Q,
    whitelisted_callers: HashSet<Principal>,
}

impl<T: TrackedAddresses, Q: NotifyMinterQueue> State<T, Q> {
    pub fn new(
        tracked_addresses: T,
        notify_minter_queue: Q,
        whitelisted_callers: HashSet<Principal>,
    ) -> Self {
        State {
            tracked_addresses,
            notify_minter_queue,
            whitelisted_callers,
        }
    }

    pub fn enable_forwarding(&mut self, evm_address: String, icp_account: IcpAccount) {
        self.tracked_addresses.push(icp_account, evm_address);
    }

    pub fn is_forwarding(&self, evm_address: &str) -> Option<IcpAccount> {
        self.tracked_addresses.get(evm_address)
    }

    pub fn push_onto_notify_minter_queue(&mut self, evm_address: EvmAddress) -> bool {
        if let Some(icp_account) = self.tracked_addresses.get(&evm_address.address) {
            self.notify_minter_queue.push(evm_address, icp_account);
            true
        } else {
            false
        }
    }

    pub fn pop_from_notify_minter_queue(&mut self) -> Option<(EvmAddress, IcpAccount)> {
        self.notify_minter_queue.pop()
    }

    pub fn caller_is_whitelisted(&self, caller: &Principal) -> bool {
        self.whitelisted_callers.contains(caller)
    }
}
