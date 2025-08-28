use crate::lib::{DefaultNotifyMinterQueue, DefaultTrackedAddresses};
use candid::{CandidType, Principal};
use ic_cdk::init;
use serde::{Deserialize, Serialize};

#[init]
fn init(args: InitOrUpgradeArgs) {
    let init_args = args.init();
    crate::lib::init(
        DefaultTrackedAddresses::default(),
        DefaultNotifyMinterQueue::default(),
        init_args.whitelisted_callers.into_iter().collect(),
    );
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum InitOrUpgradeArgs {
    Init(InitArgs),
    Upgrade(UpgradeArgs),
}

impl InitOrUpgradeArgs {
    fn init(self) -> InitArgs {
        let InitOrUpgradeArgs::Init(init) = self else {
            panic!("Not of type Init");
        };
        init
    }
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct InitArgs {
    pub whitelisted_callers: Vec<Principal>,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct UpgradeArgs {}
