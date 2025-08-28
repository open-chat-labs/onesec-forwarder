use ic_cdk::query;
use one_sec_deposit_notifier_canister_types::IsTrackingAddressArgs;

#[query]
fn is_tracking_address(args: IsTrackingAddressArgs) -> bool {
    crate::lib::is_address_tracked(&args.address)
}
