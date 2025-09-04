use crate::state::forwarding_addresses::stable::StableForwardingAddresses;
use ic_stable_structures::DefaultMemoryImpl;
use ic_stable_structures::memory_manager::VirtualMemory;
use onesec_forwarder_types::IcpAccount;

mod stable;

pub trait ForwardingAddresses {
    fn push(&mut self, icp_account: IcpAccount, evm_address: String);
    fn get(&self, evm_address: &str) -> Option<IcpAccount>;
}

pub type DefaultForwardingAddresses = StableForwardingAddresses<VirtualMemory<DefaultMemoryImpl>>;
