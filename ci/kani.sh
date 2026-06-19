#!/usr/bin/env bash
# Runs Kani proofs twice:
#   1. Normal host run (x86_64, little-endian, generic group impl via not(kani))
#   2. Simulated s390x run (big-endian cfgs injected via RUSTFLAGS, same generic impl)
#
# Usage:
#   ./ci/kani.sh                   # both passes
#   ./ci/kani.sh --host-only       # host pass only
#   ./ci/kani.sh --s390x-only      # s390x-simulated pass only

set -euo pipefail

HOST_ONLY=0
S390X_ONLY=0
for arg in "$@"; do
    case "$arg" in
        --host-only)   HOST_ONLY=1 ;;
        --s390x-only)  S390X_ONLY=1 ;;
    esac
done

KANI_FLAGS=(
    -Z function-contracts
    -Z stubbing
    --jobs 1
    --no-memory-safety-checks
)

run_host() {
    echo "=== Kani: host (x86_64, little-endian, generic group via not(kani)) ==="
    cargo kani "${KANI_FLAGS[@]}"
}

run_s390x_sim() {
    echo "=== Kani: simulated s390x (big-endian cfgs, generic group) ==="
    # Inject big-endian and 64-bit cfgs so that:
    #   - cfg!(target_endian = "big") is true  -> activates kani_proofs_s390x
    #   - cfg!(target_pointer_width = "64")    -> GroupWord stays u64
    #   - cfg!(target_arch = "arm") is false   -> trailing_zeros takes non-ARM path
    # The generic group impl is already forced by not(kani) in group/mod.rs.
    RUSTFLAGS='
        --cfg target_endian="big"
        --cfg target_pointer_width="64"
    ' cargo kani "${KANI_FLAGS[@]}"
}

if [[ "$S390X_ONLY" -eq 0 ]]; then
    run_host
fi

if [[ "$HOST_ONLY" -eq 0 ]]; then
    run_s390x_sim
fi

echo "=== All Kani passes completed ==="
