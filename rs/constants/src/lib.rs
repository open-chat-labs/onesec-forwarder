use ic_principal::Principal;

#[allow(dead_code)]
const ONESEC_MINTER_CANISTER_ID: Principal =
    Principal::from_slice(&[0, 0, 0, 0, 2, 48, 11, 124, 1, 1]);

#[test]
fn onesec_minter_canister_id() {
    assert_eq!(
        ONESEC_MINTER_CANISTER_ID,
        Principal::from_text("5okwm-giaaa-aaaar-qbn6a-cai").unwrap()
    )
}
