#!/usr/bin/env bash

here=$(dirname "$0")
# shellcheck source=multinode-demo/common.sh
source "$here"/common.sh

set -e

rm -rf "$HUIONE_CONFIG_DIR"/latest-mainnet-beta-snapshot
mkdir -p "$HUIONE_CONFIG_DIR"/latest-mainnet-beta-snapshot
(
  cd "$HUIONE_CONFIG_DIR"/latest-mainnet-beta-snapshot || exit 1
  set -x
  wget http://api.mainnet-beta.huione.com/genesis.tar.bz2
  wget --trust-server-names http://api.mainnet-beta.huione.com/snapshot.tar.bz2
)

snapshot=$(ls "$HUIONE_CONFIG_DIR"/latest-mainnet-beta-snapshot/snapshot-[0-9]*-*.tar.zst)
if [[ -z $snapshot ]]; then
  echo Error: Unable to find latest snapshot
  exit 1
fi

if [[ ! $snapshot =~ snapshot-([0-9]*)-.*.tar.zst ]]; then
  echo Error: Unable to determine snapshot slot for "$snapshot"
  exit 1
fi

snapshot_slot="${BASH_REMATCH[1]}"

rm -rf "$HUIONE_CONFIG_DIR"/bootstrap-validator
mkdir -p "$HUIONE_CONFIG_DIR"/bootstrap-validator


# Create genesis ledger
if [[ -r $FAUCET_KEYPAIR ]]; then
  cp -f "$FAUCET_KEYPAIR" "$HUIONE_CONFIG_DIR"/faucet.json
else
  $huione_keygen new --no-passphrase -fso "$HUIONE_CONFIG_DIR"/faucet.json
fi

if [[ -f $BOOTSTRAP_VALIDATOR_IDENTITY_KEYPAIR ]]; then
  cp -f "$BOOTSTRAP_VALIDATOR_IDENTITY_KEYPAIR" "$HUIONE_CONFIG_DIR"/bootstrap-validator/identity.json
else
  $huione_keygen new --no-passphrase -so "$HUIONE_CONFIG_DIR"/bootstrap-validator/identity.json
fi

$huione_keygen new --no-passphrase -so "$HUIONE_CONFIG_DIR"/bootstrap-validator/vote-account.json
$huione_keygen new --no-passphrase -so "$HUIONE_CONFIG_DIR"/bootstrap-validator/stake-account.json

$huione_ledger_tool create-snapshot \
  --ledger "$HUIONE_CONFIG_DIR"/latest-mainnet-beta-snapshot \
  --faucet-pubkey "$HUIONE_CONFIG_DIR"/faucet.json \
  --faucet-lamports 500000000000000000 \
  --bootstrap-validator "$HUIONE_CONFIG_DIR"/bootstrap-validator/identity.json \
                        "$HUIONE_CONFIG_DIR"/bootstrap-validator/vote-account.json \
                        "$HUIONE_CONFIG_DIR"/bootstrap-validator/stake-account.json \
  --hashes-per-tick sleep \
  "$snapshot_slot" "$HUIONE_CONFIG_DIR"/bootstrap-validator

$huione_ledger_tool modify-genesis \
  --ledger "$HUIONE_CONFIG_DIR"/latest-mainnet-beta-snapshot \
  --hashes-per-tick sleep \
  "$HUIONE_CONFIG_DIR"/bootstrap-validator
