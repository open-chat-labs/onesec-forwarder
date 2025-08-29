use crate::state::State;
use onesec_forwarder_types::*;
use std::cell::RefCell;

pub use crate::state::DefaultTrackedAddresses;

mod state;

thread_local! {
    static STATE: RefCell<Option<State>> = RefCell::default();
}

pub fn init(tracked_addresses: DefaultTrackedAddresses) {
    assert!(STATE.with_borrow(|s| s.is_none()));

    STATE.set(Some(State::new(tracked_addresses)));
}

pub fn enable_forwarding(icp_account: IcpAccount) {
    let evm_address = calculate_forwarding_address(&icp_account);
    with_state_mut(|s| s.enable_forwarding(evm_address, icp_account));
}

pub fn is_forwarding(evm_address: &str) -> Option<IcpAccount> {
    with_state(|s| s.is_forwarding(evm_address))
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
