export WORK_DIR=/home/circleci/
export SCCACHE_CACHE_SIZE=200M
export WORK_DIR="$WORK_DIR/.cache/sccache"
export SCCACHE_DIR="$WORK_DIR/.cache/sccache"
export PATH="$PATH:$WORK_DIR/.bin"
export RUSTC_WRAPPER="sccache"
export RUST_MIN_STACK=8388608
export RUSTFLAGS="-Clink-dead-code"
