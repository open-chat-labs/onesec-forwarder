use crate::EvmAddress;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

pub trait NotifyMinterQueue {
    fn push(&mut self, address: EvmAddress);
    fn pop(&mut self) -> Option<EvmAddress>;
}

pub type DefaultNotifyMinterQueue = HeapNotifyMinterQueue;

#[derive(Serialize, Deserialize, Default)]
pub struct HeapNotifyMinterQueue {
    queue: VecDeque<EvmAddress>,
}

impl NotifyMinterQueue for HeapNotifyMinterQueue {
    fn push(&mut self, address: EvmAddress) {
        self.queue.push_back(address);
    }

    fn pop(&mut self) -> Option<EvmAddress> {
        self.queue.pop_front()
    }
}
