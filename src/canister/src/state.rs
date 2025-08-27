use crate::PendingDepositNotification;
use crate::state::pending_deposit_notifications::{HeapPendingDepositNotifications, PendingDepositNotifications};
use crate::state::tracked_addresses::{HeapTrackedAddresses, TrackedAddresses};

mod tracked_addresses;
mod pending_deposit_notifications;

pub use tracked_addresses::DefaultTrackedAddresses;
pub use pending_deposit_notifications::DefaultPendingDepositNotifier;

pub struct State<T = DefaultTrackedAddresses, D = DefaultPendingDepositNotifier> {
    tracked_addresses: T,
    pending_deposit_notifications: D,
}

impl<T: TrackedAddresses, D: PendingDepositNotifications> State<T, D> {
    pub fn new(tracked_addresses: T, pending_deposit_notifications: D) -> Self {
        State { tracked_addresses, pending_deposit_notifications }
    }

    pub fn track_address(&mut self, address: String) {
        self.tracked_addresses.push(address);
    }

    pub fn is_tracked_address(&self, address: &str) -> bool {
        self.tracked_addresses.contains(address)
    }

    pub fn push_pending_deposit_notification(&mut self, notification: PendingDepositNotification) {
        self.pending_deposit_notifications.push(notification);
    }

    pub fn pop_pending_deposit_notification(&mut self) -> Option<PendingDepositNotification> {
        self.pending_deposit_notifications.pop()
    }
}
