use crate::EnableForwardingArgs;
use ic_cdk::update;

#[update]
fn enable_forwarding(args: EnableForwardingArgs) -> String {
    crate::lib::enable_forwarding(args.icp_account)
}
