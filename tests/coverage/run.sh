# Need to install:
#   1. grcov (from your package manager or run 'cargo install grcov')
#   2. llvm-tools (rustup component add llvm-tools-preview)

rm -rf ./target/coverage

CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='cargo-test-%p-%m.profraw' cargo test
grcov . --binary-path ./target/debug/deps/ -s . -t html --branch \
  --ignore-not-existing \
  --ignore '../*' \
  --ignore '/*' \
  --ignore 'target/debug/*' \
  -o target/coverage/html

grcov . --binary-path ./target/debug/deps/ -s . -t lcov --branch \
  --ignore-not-existing \
  --ignore '../*' \
  --ignore '/*' \
  --ignore 'target/debug/*' \
  -o target/coverage/tests.lcov

rm ./cargo-test*
