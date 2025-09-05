use crate::state::forwarding_addresses::ForwardingAddresses;
use candid::CandidType;
use ic_stable_structures::storable::Bound;
use ic_stable_structures::{Memory, StableBTreeMap, Storable};
use onesec_forwarder_types::IcpAccount;
use serde::Deserialize;
use std::borrow::Cow;

pub struct StableForwardingAddresses<M: Memory> {
    addresses: StableBTreeMap<String, StorableIcpAccount, M>,
}

impl<M: Memory> StableForwardingAddresses<M> {
    pub fn init(memory: M) -> Self {
        StableForwardingAddresses {
            addresses: StableBTreeMap::init(memory),
        }
    }
}

#[derive(CandidType, Deserialize, Clone)]
struct StorableIcpAccount(IcpAccount);

impl<M: Memory> ForwardingAddresses for StableForwardingAddresses<M> {
    fn push(&mut self, icp_account: IcpAccount, evm_address: String) {
        self.addresses
            .insert(evm_address.to_lowercase(), StorableIcpAccount(icp_account));
    }

    fn get(&self, evm_address: &str) -> Option<IcpAccount> {
        self.addresses.get(&evm_address.to_lowercase()).map(|v| v.0)
    }
}

impl Storable for StorableIcpAccount {
    fn into_bytes(self) -> Vec<u8> {
        candid::encode_one(self).unwrap()
    }

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}
