SCRIPT=$(readlink -f "$0")
SCRIPT_DIR=$(dirname "$SCRIPT")
cd $SCRIPT_DIR

./generate-candid.sh one_sec_forwarder_canister > ../rs/canister/can.did