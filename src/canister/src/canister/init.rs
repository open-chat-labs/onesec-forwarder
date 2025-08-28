use crate::InitOrUpgradeArgs;
use crate::lib::{DefaultNotifyMinterQueue, DefaultTrackedAddresses};
use ic_cdk::init;

#[init]
fn init(args: InitOrUpgradeArgs) {
    let init_args = args.init();
    crate::lib::init(
        DefaultTrackedAddresses::default(),
        DefaultNotifyMinterQueue::default(),
        init_args.whitelisted_callers.into_iter().collect(),
    );
}
