use std::collections::VecDeque;
use serde::{Deserialize, Serialize};
use crate::PendingDepositNotification;

pub trait PendingDepositNotifications {
    fn push(&mut self, notification: PendingDepositNotification);
    fn pop(&mut self) -> Option<PendingDepositNotification>;
}

pub type DefaultPendingDepositNotifier = HeapPendingDepositNotifications;

#[derive(Serialize, Deserialize, Default)]
pub struct HeapPendingDepositNotifications {
    queue: VecDeque<PendingDepositNotification>,
}

impl PendingDepositNotifications for HeapPendingDepositNotifications {
    fn push(&mut self, notification: PendingDepositNotification) {
        self.queue.push_back(notification);
    }

    fn pop(&mut self) -> Option<PendingDepositNotification> {
        self.queue.pop_front()
    }
}