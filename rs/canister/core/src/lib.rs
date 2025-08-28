use crate::state::State;
use ic_principal::Principal;
use onesec_forwarder_types::*;
use std::cell::RefCell;
use std::collections::HashSet;

pub use crate::state::{DefaultNotifyMinterQueue, DefaultTrackedAddresses};

mod state;

thread_local! {
    static STATE: RefCell<Option<State>> = RefCell::default();
}

pub fn init(
    tracked_addresses: DefaultTrackedAddresses,
    notify_minter_queue: DefaultNotifyMinterQueue,
    whitelisted_callers: HashSet<Principal>,
) {
    assert!(STATE.with_borrow(|s| s.is_none()));

    STATE.set(Some(State::new(
        tracked_addresses,
        notify_minter_queue,
        whitelisted_callers,
    )));
}

pub fn enable_forwarding(icp_account: IcpAccount) {
    let evm_address = calculate_forwarding_address(&icp_account);
    with_state_mut(|s| s.enable_forwarding(evm_address, icp_account));
}

pub fn is_forwarding(evm_address: &str) -> bool {
    with_state(|s| s.is_forwarding(evm_address).is_some())
}

pub fn push_onto_notify_minter_queue(evm_address: EvmAddress) -> bool {
    with_state_mut(|s| s.push_onto_notify_minter_queue(evm_address))
}

pub fn pop_from_notify_minter_queue() -> Option<(EvmAddress, IcpAccount)> {
    with_state_mut(|s| s.pop_from_notify_minter_queue())
}

pub fn caller_is_whitelisted(caller: &Principal) -> bool {
    with_state(|s| s.caller_is_whitelisted(caller))
}

fn calculate_forwarding_address(_icp_account: &IcpAccount) -> String {
    "TODO".to_string()
}

fn with_state<F: FnOnce(&State) -> T, T>(f: F) -> T {
    STATE.with_borrow(|s| f(s.as_ref().unwrap()))
}

fn with_state_mut<F: FnOnce(&mut State) -> T, T>(f: F) -> T {
    STATE.with_borrow_mut(|s| f(s.as_mut().unwrap()))
}
