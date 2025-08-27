use crate::lib::{DefaultPendingDepositNotifier, DefaultTrackedAddresses};
use ic_cdk::init;

#[init]
fn init() {
    crate::lib::init(
        DefaultTrackedAddresses::default(),
        DefaultPendingDepositNotifier::default(),
    );
}
