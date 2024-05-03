set quiet

# test the library with all features
test:
  just install-nextest
  echo "testing..."
  cargo nextest run -F full
  echo "done!"


# benchmark the js and rust vbml implementations
bench:
  echo "benchmarking js and rust..."
  just bench-js-native
  just bench-rs-native
  just interpret
  echo "done!"


# benchmark the js vbml implementations
bench-js:
  echo "benchmarking js..."
  just bench-js-native


# benchmark the rust vbml implementations
bench-rs:
  just install-cargo-criterion
  echo "benchmarking rust..."
  just bench-rs-native

# benchmark the rust vbml implementations with flamegraph
flame $CARGO_PROFILE_BENCH_DEBUG="true":
  just install-flamegraph
  echo "profiling rust..."
  cargo flamegraph -o benchmark/out/flamegraph.svg --open --bench vbml -- --bench
  echo "done!"


# interpret the js and rust results
interpret:
  echo "interpreting results..."
  cargo run --example interpret


[private]
bench-js-native:
  #!/usr/bin/env bash
  cd benchmark/js
  echo "installing dependencies..."
  npm install &> /dev/null
  echo "running js benchmarks..."
  npm run bench


[private]
bench-rs-native:
  echo "running rust benchmarks..."
  cargo criterion --message-format=json > /tmp/vestaboard-rs-criterion.json
  jq -s < /tmp/vestaboard-rs-criterion.json > benchmark/out/rust.json


[private]
install-cargo-criterion:
  #!/usr/bin/env bash
  if ! command -v cargo-criterion &> /dev/null; then
    echo "installing cargo-criterion..."
    cargo install cargo-criterion
  fi

[private]
install-flamegraph:
  #!/usr/bin/env bash
  if ! command -v flamegraph &> /dev/null; then
    echo "installing flamegraph..."
    cargo install flamegraph
  fi

[private]
install-nextest:
  #!/usr/bin/env bash
  if ! command -v nextest &> /dev/null; then
    echo "installing nextest..."
    cargo install nextest
  fi