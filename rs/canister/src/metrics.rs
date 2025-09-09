use ic_metrics_encoder::MetricsEncoder;

pub fn encode_metrics(w: &mut MetricsEncoder<Vec<u8>>) -> std::io::Result<()> {
    w.encode_gauge("heap_memory_bytes", heap_size() as f64, "")?;
    w.encode_gauge(
        "stable_memory_bytes",
        (ic_cdk::api::stable_size() * ic_cdk::stable::WASM_PAGE_SIZE_IN_BYTES) as f64,
        "",
    )?;
    w.encode_gauge(
        "cycles_balance",
        ic_cdk::api::canister_cycle_balance() as f64,
        "",
    )?;
    w.encode_gauge(
        "liquid_cycles_balance",
        ic_cdk::api::canister_liquid_cycle_balance() as f64,
        "",
    )?;
    w.encode_counter(
        "forwarding_addresses",
        crate::lib::forwarding_addresses_len() as f64,
        "",
    )?;

    Ok(())
}

fn heap_size() -> u64 {
    #[cfg(target_arch = "wasm32")]
    {
        core::arch::wasm32::memory_size(0) as u64 * 65536
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // This branch won't actually ever be taken
        0
    }
}
