use crate::state::State;
pub use crate::state::{DefaultNotifyMinterQueue, DefaultTrackedAddresses};
use candid::Principal;
use one_sec_deposit_notifier_types::EvmAddress;
use std::cell::RefCell;
use std::collections::HashSet;

mod state;

const ONE_SEC_MINTER_CANISTER_ID: Principal =
    Principal::from_slice(&[0, 0, 0, 0, 2, 48, 11, 124, 1, 1]);

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

pub fn track_address(address: String) {
    with_state_mut(|s| s.track_address(address));
}

pub fn is_address_tracked(address: &str) -> bool {
    with_state(|s| s.is_tracked_address(address))
}

pub fn push_address_onto_notify_minter_queue(address: EvmAddress) {
    with_state_mut(|s| s.push_address_onto_notify_minter_queue(address));
}

pub fn pop_address_from_notify_minter_queue() -> Option<EvmAddress> {
    with_state_mut(|s| s.pop_address_from_notify_minter_queue())
}

pub fn caller_is_whitelisted(caller: &Principal) -> bool {
    with_state(|s| s.caller_is_whitelisted(caller))
}

fn with_state<F: FnOnce(&State) -> T, T>(f: F) -> T {
    STATE.with_borrow(|s| f(s.as_ref().unwrap()))
}

fn with_state_mut<F: FnOnce(&mut State) -> T, T>(f: F) -> T {
    STATE.with_borrow_mut(|s| f(s.as_mut().unwrap()))
}

#[test]
fn one_sec_minter_canister_id() {
    assert_eq!(
        ONE_SEC_MINTER_CANISTER_ID,
        Principal::from_text("5okwm-giaaa-aaaar-qbn6a-cai").unwrap()
    )
}
