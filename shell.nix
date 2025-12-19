{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  name = "bufsy-dev-shell";

  # Provide Rust toolchain: rustc and cargo
  buildInputs = [
    # pkgs.rustc
    # pkgs.cargo
    pkgs.pkg-config
    pkgs.openssl
  ];

  # Useful environment defaults for development
  # RUST_BACKTRACE = "1";
  # CARGO_TERM_COLOR = "always";

  # Keep cargo/rustup state inside the repository to avoid polluting the user profile.
  # This is optional but convenient for reproducible local shells.
  # shellHook = ''
  #   export CARGO_HOME="$PWD/.cargo"
  #   export RUSTUP_HOME="$PWD/.rustup"

  #   echo "Entered bufsy development shell"
  #   echo "Rust: $(rustc --version 2>/dev/null || echo 'rustc not found')"
  #   echo "Cargo: $(cargo --version 2>/dev/null || echo 'cargo not found')"
  # '';
}
