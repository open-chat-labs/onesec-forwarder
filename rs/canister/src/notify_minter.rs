use crate::NotifyMinterArgs;
use crate::guards::caller_is_whitelisted;
use ic_cdk::update;

#[update(guard = "caller_is_whitelisted")]
fn notify_minter(args: NotifyMinterArgs) {
    for address in args.addresses {
        crate::lib::push_address_onto_notify_minter_queue(address);
    }
}
