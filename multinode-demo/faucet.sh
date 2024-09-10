#!/usr/bin/env bash
#
# Starts an instance of huione-faucet
#
here=$(dirname "$0")

# shellcheck source=multinode-demo/common.sh
source "$here"/common.sh

[[ -f "$HUIONE_CONFIG_DIR"/faucet.json ]] || {
  echo "$HUIONE_CONFIG_DIR/faucet.json not found, create it by running:"
  echo
  echo "  ${here}/setup.sh"
  exit 1
}

set -x
# shellcheck disable=SC2086 # Don't want to double quote $huione_faucet
exec $huione_faucet --keypair "$HUIONE_CONFIG_DIR"/faucet.json "$@"
