build_test_clippy(){
  while read data; do
    TOML="$data"/Cargo.toml
    printf "Project: %s\n" "$TOML"
    cargo build --verbose --manifest-path "$TOML"
    cargo test --verbose --manifest-path "$TOML"
    cargo clippy --verbose --manifest-path "$TOML"
  done
}

find . -name 'Cargo.toml' -printf '%h\n' | sort -u | build_test_clippy


