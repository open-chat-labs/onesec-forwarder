use crate::lib::{DefaultNotifyMinterQueue, DefaultTrackedAddresses};
use ic_cdk::init;
use one_sec_deposit_notifier_canister_types::InitOrUpgradeArgs;

#[init]
fn init(args: InitOrUpgradeArgs) {
    let init_args = args.init();
    crate::lib::init(
        DefaultTrackedAddresses::default(),
        DefaultNotifyMinterQueue::default(),
        init_args.whitelisted_callers.into_iter().collect(),
    );
}
