source ./setup-environment.sh
cd /home/circleci/project/ || exit
cargo test --all -- --skip dpc --skip dpc_integration_test --skip startup_handshake_stored_peers
