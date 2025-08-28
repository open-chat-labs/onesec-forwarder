pub fn caller_is_whitelisted() -> Result<(), String> {
    let caller = ic_cdk::api::msg_caller();

    if crate::lib::caller_is_whitelisted(&caller) {
        Ok(())
    } else {
        Err("Not authorized".to_string())
    }
}
