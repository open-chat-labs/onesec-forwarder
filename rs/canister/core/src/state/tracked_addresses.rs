use onesec_forwarder_types::IcpAccount;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub trait TrackedAddresses {
    fn push(&mut self, icp_account: IcpAccount, evm_address: String);
    fn get(&self, evm_address: &str) -> Option<IcpAccount>;
}

pub type DefaultTrackedAddresses = HeapTrackedAddresses;

#[derive(Serialize, Deserialize, Default)]
pub struct HeapTrackedAddresses {
    addresses: BTreeMap<String, IcpAccount>,
}

impl TrackedAddresses for HeapTrackedAddresses {
    fn push(&mut self, icp_account: IcpAccount, evm_address: String) {
        self.addresses.insert(evm_address, icp_account);
    }

    fn get(&self, evm_address: &str) -> Option<IcpAccount> {
        self.addresses.get(evm_address).cloned()
    }
}
