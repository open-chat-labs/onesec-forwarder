use crate::IsForwardingAddressArgs;
use ic_cdk::query;

#[query]
fn is_forwarding_address(args: IsForwardingAddressArgs) -> bool {
    crate::lib::is_forwarding_address(&args.evm_address).is_some()
}
