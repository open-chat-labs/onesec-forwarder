use crate::IsForwardingArgs;
use ic_cdk::query;

#[query]
fn is_forwarding(args: IsForwardingArgs) -> bool {
    crate::lib::is_forwarding(&args.evm_address).is_some()
}
