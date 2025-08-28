use crate::IsTrackingAddressArgs;
use ic_cdk::query;

#[query]
fn is_tracking_address(args: IsTrackingAddressArgs) -> bool {
    crate::lib::is_address_tracked(&args.address)
}
