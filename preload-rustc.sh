#!/usr/bin/env bash
# Wrapper to preload rustc_driver.so to avoid TLS issues

RUSTC_SO=$(find $(rustc --print sysroot)/lib -name "librustc_driver-*.so" | head -1)

if [ -z "$RUSTC_SO" ]; then
    echo "Error: Could not find librustc_driver-*.so"
    exit 1
fi

export RUSTC_DRIVER_SO="$RUSTC_SO"
export LD_PRELOAD="$RUSTC_SO:$LD_PRELOAD"

exec "$@"
