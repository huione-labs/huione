# |source| this file
#
# Common utilities shared by other scripts in this directory
#
# The following directive disable complaints about unused variables in this
# file:
# shellcheck disable=2034
#

# shellcheck source=net/common.sh
source "$(cd "$(dirname "${BASH_SOURCE[0]}")"/.. || exit 1; pwd)"/net/common.sh

prebuild=
if [[ $1 = "--prebuild" ]]; then
  prebuild=true
fi

if [[ $(uname) != Linux ]]; then
  # Protect against unsupported configurations to prevent non-obvious errors
  # later. Arguably these should be fatal errors but for now prefer tolerance.
  if [[ -n $HUIONE_CUDA ]]; then
    echo "Warning: CUDA is not supported on $(uname)"
    HUIONE_CUDA=
  fi
fi

if [[ -n $USE_INSTALL || ! -f "$HUIONE_ROOT"/Cargo.toml ]]; then
  huione_program() {
    declare program="$1"
    if [[ -z $program ]]; then
      printf "huione"
    else
      printf "huione-%s" "$program"
    fi
  }
else
  huione_program() {
    declare program="$1"
    declare crate="$program"
    if [[ -z $program ]]; then
      crate="cli"
      program="huione"
    else
      program="huione-$program"
    fi

    if [[ -n $NDEBUG ]]; then
      maybe_release=--release
    fi

    # Prebuild binaries so that CI sanity check timeout doesn't include build time
    if [[ $prebuild ]]; then
      (
        set -x
        # shellcheck disable=SC2086 # Don't want to double quote
        cargo $CARGO_TOOLCHAIN build $maybe_release --bin $program
      )
    fi

    printf "cargo $CARGO_TOOLCHAIN run $maybe_release  --bin %s %s -- " "$program"
  }
fi

huione_bench_tps=$(huione_program bench-tps)
huione_faucet=$(huione_program faucet)
huione_validator=$(huione_program validator)
huione_validator_cuda="$huione_validator --cuda"
huione_genesis=$(huione_program genesis)
huione_gossip=$(huione_program gossip)
huione_keygen=$(huione_program keygen)
huione_ledger_tool=$(huione_program ledger-tool)
huione_cli=$(huione_program)

export RUST_BACKTRACE=1

default_arg() {
  declare name=$1
  declare value=$2

  for arg in "${args[@]}"; do
    if [[ $arg = "$name" ]]; then
      return
    fi
  done

  if [[ -n $value ]]; then
    args+=("$name" "$value")
  else
    args+=("$name")
  fi
}

replace_arg() {
  declare name=$1
  declare value=$2

  default_arg "$name" "$value"

  declare index=0
  for arg in "${args[@]}"; do
    index=$((index + 1))
    if [[ $arg = "$name" ]]; then
      args[$index]="$value"
    fi
  done
}
