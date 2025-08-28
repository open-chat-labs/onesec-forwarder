use crate::NotifyMinterArgs;
use crate::guards::caller_is_whitelisted;
use ic_cdk::update;

#[update(guard = "caller_is_whitelisted")]
fn notify_minter(args: NotifyMinterArgs) {
    for evm_address in args.evm_addresses {
        crate::lib::push_onto_notify_minter_queue(evm_address);
    }
}
