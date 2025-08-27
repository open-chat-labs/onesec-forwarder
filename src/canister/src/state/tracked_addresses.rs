use std::collections::BTreeSet;
use candid::Principal;
use serde::{Deserialize, Serialize};

pub trait TrackedAddresses {
    fn push(&mut self, address: String);
    fn contains(&self, address: &str) -> bool;
}

pub type DefaultTrackedAddresses = HeapTrackedAddresses;

#[derive(Serialize, Deserialize, Default)]
pub struct HeapTrackedAddresses {
    addresses: BTreeSet<String>,
}

impl TrackedAddresses for HeapTrackedAddresses {
    fn push(&mut self, address: String) {
        self.addresses.insert(address);
    }

    fn contains(&self, address: &str) -> bool {
        self.addresses.contains(address)
    }
}