use std::cell::RefCell;
use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};
use crate::state::{DefaultPendingDepositNotifier, DefaultTrackedAddresses, State};

mod state;

const ONE_SEC_MINTER_CANISTER_ID: Principal = Principal::from_slice(&[0, 0, 0, 0, 2, 48, 11, 124, 1, 1]);

thread_local! {
    static STATE: RefCell<Option<State>> = RefCell::default();
}

pub fn init(tracked_addresses: DefaultTrackedAddresses, pending_deposit_notifications: DefaultPendingDepositNotifier) {
    assert!(STATE.with_borrow(|s| s.is_none()));

    STATE.set(Some(State::new(tracked_addresses, pending_deposit_notifications)));
}

pub fn track_address(address: String) {
    with_state_mut(|s| s.track_address(address));
}

pub fn is_address_tracked(address: &str) -> bool {
    with_state(|s| s.is_tracked_address(address))
}

fn with_state<F: FnOnce(&State) -> T, T>(f: F) -> T {
    STATE.with_borrow(|s| f(s.as_ref().unwrap()))
}

fn with_state_mut<F: FnOnce(&mut State) -> T, T>(f: F) -> T {
    STATE.with_borrow_mut(|s| f(s.as_mut().unwrap()))
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
struct PendingDepositNotification {
    chain: EvmChain,
    address: String,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
enum EvmChain {
    Ethereum,
    Arbitrum,
    Base,
}

trait Runtime {
    fn call<A: CandidType, R: >(&self, canister_id: Principal, method_name: &str, args: A) -> impl Future<Output = R>;
}

#[test]
fn one_sec_minter_canister_id() {
    assert_eq!(
        ONE_SEC_MINTER_CANISTER_ID,
        Principal::from_text("5okwm-giaaa-aaaar-qbn6a-cai").unwrap()
    )
}
