use crate::EvmAddress;
use crate::state::notify_minter_queue::NotifyMinterQueue;
use crate::state::tracked_addresses::TrackedAddresses;
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

    pub fn track_address(&mut self, address: String) {
        self.tracked_addresses.push(address);
    }

    pub fn is_tracked_address(&self, address: &str) -> bool {
        self.tracked_addresses.contains(address)
    }

    pub fn push_address_onto_notify_minter_queue(&mut self, address: EvmAddress) {
        self.notify_minter_queue.push(address);
    }

    pub fn pop_address_from_notify_minter_queue(&mut self) -> Option<EvmAddress> {
        self.notify_minter_queue.pop()
    }

    pub fn caller_is_whitelisted(&self, caller: &Principal) -> bool {
        self.whitelisted_callers.contains(caller)
    }
}
