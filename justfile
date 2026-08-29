# Recipes for building and flashing the firmware. Run `just` to list
# them.

ESPUP_VERSION := "0.17.1"
ESPFLASH_VERSION := "4.4.0"

_default:
    @just --list --unsorted

# Install espup, the esp toolchain, and espflash
[group('setup')]
setup-tools:
    @if command -v espup >/dev/null; then \
        echo "espup already installed; using the system copy"; \
    else \
        RUSTUP_TOOLCHAIN=stable cargo install --locked espup --version {{ESPUP_VERSION}}; \
    fi
    @if [ -d "$(rustup show home)/toolchains/esp" ]; then \
        echo "esp toolchain already installed; run 'espup update' to upgrade it"; \
    else \
        f=$(mktemp) \
        && { espup install --targets esp32s3 --export-file "$f"; s=$?; rm -f "$f"; exit $s; }; \
    fi
    @if command -v cargo-espflash >/dev/null; then \
        echo "cargo-espflash already installed; using the system copy"; \
    else \
        RUSTUP_TOOLCHAIN=stable cargo install --locked cargo-espflash --version {{ESPFLASH_VERSION}}; \
    fi

# Build the release firmware
[group('dev')]
build:
    cargo build --release

# Build and flash the release firmware over USB (then press RESET)
[group('dev')]
flash: (_require "cargo-espflash")
    cargo espflash flash --release --chip esp32s3

[private]
_require tool:
    @command -v {{tool}} >/dev/null || { \
        echo "error: {{tool}} is not installed; run 'just setup-tools'" >&2; \
        exit 1; }
