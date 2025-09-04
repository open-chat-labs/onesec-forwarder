use ic_stable_structures::{
    DefaultMemoryImpl,
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
};
use std::sync::LazyLock;

const FORWARDING_ADDRESSES: MemoryId = MemoryId::new(1);

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

const MEMORY_MANAGER: LazyLock<MemoryManager<DefaultMemoryImpl>> =
    LazyLock::new(|| MemoryManager::init_with_bucket_size(DefaultMemoryImpl::default(), 4));

pub fn get_forwarding_addresses_memory() -> Memory {
    get_memory(FORWARDING_ADDRESSES)
}

fn get_memory(id: MemoryId) -> Memory {
    MEMORY_MANAGER.get(id)
}
