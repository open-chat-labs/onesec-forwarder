use crate::IcpAccount;
use crate::state::forwarding_addresses::ForwardingAddresses;

mod forwarding_addresses;

pub use forwarding_addresses::DefaultForwardingAddresses;

pub struct State<T = DefaultForwardingAddresses> {
    forwarding_addresses: T,
}

impl<T: ForwardingAddresses> State<T> {
    pub fn new(forwarding_addresses: T) -> Self {
        State {
            forwarding_addresses,
        }
    }

    pub fn enable_forwarding(&mut self, evm_address: String, icp_account: IcpAccount) {
        self.forwarding_addresses.push(icp_account, evm_address);
    }

    pub fn is_forwarding_address(&self, evm_address: &str) -> Option<IcpAccount> {
        self.forwarding_addresses.get(evm_address)
    }

    pub fn forwarding_addresses_len(&self) -> u64 {
        self.forwarding_addresses.len()
    }
}
