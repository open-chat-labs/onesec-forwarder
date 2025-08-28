use ic_cdk::update;
use one_sec_deposit_notifier_canister_types::TrackAddressArgs;

#[update]
fn track_address(args: TrackAddressArgs) {
    crate::lib::track_address(args.address);
}
