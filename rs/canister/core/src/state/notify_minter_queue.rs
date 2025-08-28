use crate::{EvmAddress, IcpAccount};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub trait NotifyMinterQueue {
    fn push(&mut self, evm_address: EvmAddress, icp_account: IcpAccount);
    fn pop(&mut self) -> Option<(EvmAddress, IcpAccount)>;
}

pub type DefaultNotifyMinterQueue = HeapNotifyMinterQueue;

#[derive(Serialize, Deserialize, Default)]
pub struct HeapNotifyMinterQueue {
    queue: VecDeque<(EvmAddress, IcpAccount)>,
}

impl NotifyMinterQueue for HeapNotifyMinterQueue {
    fn push(&mut self, evm_address: EvmAddress, icp_account: IcpAccount) {
        self.queue.push_back((evm_address, icp_account));
    }

    fn pop(&mut self) -> Option<(EvmAddress, IcpAccount)> {
        self.queue.pop_front()
    }
}
