use crate::TrackAddressArgs;
use ic_cdk::update;

#[update]
fn track_address(args: TrackAddressArgs) {
    crate::lib::track_address(args.address);
}
